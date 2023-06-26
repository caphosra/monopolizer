use calamine::{open_workbook, DataType, Range, RangeDeserializerBuilder, Reader, Xlsx};
use serde::{Deserialize, Serialize};

use crate::board::{Board, GameSession};
use crate::player::{Player, PlayerState};
use crate::strategy::{ExpensiveHousesProtectionStrategy, PlayerStrategy};

///
/// Holds information of a game in a serializable format.
///
#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub turn: usize,
    pub players: Vec<PlayerInfo>,
    pub places: Vec<PlaceInfo>,
}

///
/// Holds information of a player in a serializable format.
///
#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    pub player_id: usize,
    pub money: u32,
    pub is_bankrupted: bool,
    pub jail_turn: i8,
    pub position: usize,
}

///
/// Holds information of a place in a serializable format.
///
#[derive(Serialize, Deserialize)]
pub struct PlaceInfo {
    pub place_id: usize,
    pub place_name: String,
    pub owner: i32,
    pub is_mortgaged: bool,
    pub houses: i32,
}

impl Into<PlayerInfo> for (i64, i64, String, i64, i64) {
    fn into(self) -> PlayerInfo {
        let (player_id, money, is_bankrupted, jail_turn, position) = self;

        PlayerInfo {
            player_id: player_id as usize,
            money: money as u32,
            is_bankrupted: is_bankrupted == "yes",
            jail_turn: jail_turn as i8,
            position: position as usize,
        }
    }
}

impl GameSession {
    ///
    /// Reconstructs a game session from JSON.
    ///
    pub fn from_json(json: &str) -> Self {
        let game_info: GameInfo = serde_json::from_str(json).unwrap();
        let mut game = GameSession::new(game_info.players.len() as u32);

        let mut players = Vec::new();
        for player_info in game_info.players {
            players.push(Player::from_info(
                player_info,
                ExpensiveHousesProtectionStrategy::new(),
            ));
        }
        game.players = players;

        game.turn = game_info.turn;
        game.board = Board::from_infos(game_info.places);

        game
    }

    ///
    /// Parses information of the game session into a text in JSON.
    ///
    pub fn to_json(&self) -> String {
        let turn = self.turn;
        let players = self
            .players
            .iter()
            .map(|player| player.get_info())
            .collect::<Vec<_>>();
        let places = self.board.get_infos();

        let game_info = GameInfo {
            turn,
            players,
            places,
        };

        serde_json::to_string_pretty(&game_info).unwrap()
    }

    ///
    /// Imports information of a game from a .xlsx file.
    ///
    pub fn from_excel(file_name: &str) -> Self {
        let mut workbook: Xlsx<_> = open_workbook(file_name).unwrap();

        let turn = workbook.worksheet_range("Turn").unwrap().unwrap();
        let turn = turn.get_value((0, 1)).unwrap().as_i64().unwrap() as usize;

        let players_sheet = workbook.worksheet_range("Players").unwrap().unwrap();
        let players_sheet = RangeDeserializerBuilder::new()
            .from_range(&players_sheet)
            .unwrap();
        let players: Vec<Player> = players_sheet
            .into_iter()
            .map(|player| {
                let player_info: (i64, i64, String, i64, i64) = player.unwrap();

                Player::from_info(player_info.into(), ExpensiveHousesProtectionStrategy::new())
            })
            .collect();

        let places_sheet = workbook.worksheet_range("Places").unwrap().unwrap();
        let board = Board::from_rows(places_sheet);

        let mut game = GameSession::new(players.len() as u32);
        game.players = players;
        game.turn = turn;
        game.board = board;

        game
    }
}

impl Player {
    ///
    /// Retrieves the player data from `PlayerInfo`.
    ///
    pub fn from_info(info: PlayerInfo, strategy: Box<dyn PlayerStrategy + Send>) -> Self {
        let mut player = Player::new(info.player_id, strategy);
        player.money = info.money;

        if info.is_bankrupted {
            player.state = PlayerState::Bankrupted;

            assert_eq!(info.jail_turn, -1);
        } else {
            if info.jail_turn >= 0 {
                player.state = PlayerState::InJail(info.jail_turn as u8);
            } else {
                player.state = PlayerState::None;
            }
        }

        player.position = info.position;

        player
    }

    ///
    /// Turns the player into `PlayerInfo`, which is serializable.
    ///
    pub fn get_info(&self) -> PlayerInfo {
        let (is_bankrupted, jail_turn) = match self.state {
            PlayerState::None => (false, -1),
            PlayerState::Bankrupted => (true, -1),
            PlayerState::InJail(turn) => (false, turn as i8),
        };

        PlayerInfo {
            player_id: self.player_id,
            money: self.money,
            is_bankrupted,
            jail_turn,
            position: self.position,
        }
    }
}

impl Board {
    ///
    /// Retrieves the board data from a list of `PlaceInfo`.
    ///
    pub fn from_infos(infos: Vec<PlaceInfo>) -> Self {
        let mut board = Board::new();

        for info in infos {
            let place = &mut board.places[info.place_id];

            assert_eq!(place.get_place_name(), &info.place_name);

            if info.owner >= 0 {
                place.set_owner(Some(info.owner as usize));
            }
            if info.houses >= 0 {
                place.set_num_houses(info.houses as u8);
            }
            place.set_mortgaged(info.is_mortgaged);
        }

        board
    }

    pub fn from_rows(rows: Range<DataType>) -> Self {
        let places_data = RangeDeserializerBuilder::new().from_range(&rows).unwrap();
        let places = places_data
            .into_iter()
            .map(|place| {
                let (id, name, owner, is_mortgaged, houses): (i64, String, i64, String, i64) =
                    place.unwrap();

                PlaceInfo {
                    place_id: id as usize,
                    place_name: name,
                    owner: owner as i32,
                    is_mortgaged: is_mortgaged == "yes",
                    houses: houses as i32,
                }
            })
            .collect();

        Board::from_infos(places)
    }

    ///
    /// Retrieves information of places form the board.
    ///
    pub fn get_infos(&self) -> Vec<PlaceInfo> {
        let mut infos = Vec::new();
        for place in &self.places {
            if place.is_property() {
                infos.push(PlaceInfo {
                    place_id: place.get_id(),
                    place_name: place.get_place_name().to_string(),
                    owner: place.get_owner().map_or(-1, |owner| owner as i32),
                    is_mortgaged: place.is_mortgaged(),
                    houses: place.get_num_houses().map_or(-1, |owner| owner as i32),
                });
            }
        }

        infos
    }
}
