use serde::{Deserialize, Serialize};

use crate::board::{Board, MonopolyGame};
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

impl MonopolyGame {
    ///
    /// Reconstructs a game session from JSON.
    ///
    pub fn from_json(json: &str) -> Self {
        let game_info: GameInfo = serde_json::from_str(json).unwrap();
        let mut game = MonopolyGame::new(game_info.players.len() as u32);

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
}

impl Player {
    ///
    /// Retrieves the player data from `PlayerInfo`.
    ///
    pub fn from_info(info: PlayerInfo, strategy: Box<dyn PlayerStrategy>) -> Self {
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
