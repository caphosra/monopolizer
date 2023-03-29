use crate::dice_rolling::{DiceResult, DiceRolling};
use crate::events::EventKind;
use crate::places::{get_place_list, BoardColor, BoardPlace};
use crate::player::{Player, PlayerState};
use crate::strategy::ExpensiveHousesProtectionStrategy;

///
/// A place id of Jail.
///
/// In details, it determines the place where the player is sent when they complete their term.
///
const JAIL_POSITION: usize = 10;

///
/// Represents a game.
///
/// Regenerating an instance for each game is needed.
///
pub struct MonopolyGame {
    pub players: Vec<Player>,
    pub board: Board,
    pub turn: usize,
    pub logs: Vec<String>,
}

///
/// Log a formatted text inside `MonopolyGame`.
///
macro_rules! game_log {
    ($self:tt, $fmt:tt,$($x:expr),*) => {
        $self.logs.push(format!($fmt, $($x,)*));
    };
}

impl MonopolyGame {
    ///
    /// Generates a game.
    ///
    pub fn new(player_num: u32) -> Self {
        let players = (0..player_num)
            .map(|id| Player::new(id as usize, ExpensiveHousesProtectionStrategy::new()))
            .collect::<Vec<_>>();

        MonopolyGame {
            players,
            board: Board::new(),
            turn: 0,
            logs: Vec::new(),
        }
    }

    ///
    /// Gets a player as mutable by id.
    ///
    pub fn get_player_mut(&mut self, id: usize) -> &mut Player {
        assert!(id < self.players.len());

        &mut self.players[id]
    }

    ///
    /// Gets a player by id.
    ///
    pub fn get_player(&self, id: usize) -> &Player {
        assert!(id < self.players.len());

        &self.players[id]
    }

    ///
    /// Gets a number of active players.
    ///
    /// In this context, the word "active" means "not being bankrupted".
    /// So, the player who is in the jail will be counted as an active one.
    ///
    pub fn count_active_players(&self) -> usize {
        self.players
            .iter()
            .filter(|player| match player.state {
                PlayerState::Bankrupted => false,
                _ => true,
            })
            .count()
    }

    ///
    /// Gets a player whose turn it is.
    ///
    pub fn get_current_player(&self) -> &Player {
        self.get_player(self.turn as usize)
    }

    ///
    /// Gets a player whose turn it is as mutable.
    ///
    pub fn get_current_player_mut(&mut self) -> &mut Player {
        self.get_player_mut(self.turn as usize)
    }

    ///
    /// Invokes the event.
    ///
    pub fn invoke_event(&mut self, event: EventKind) {
        let turn = self.turn;
        match event {
            EventKind::None(msg) => {
                game_log!(self, "[PLAYER{}] {}", turn, msg);
            }
            EventKind::PayToBank(msg, dollars) => {
                game_log!(
                    self,
                    "[PLAYER{}] Pays ${} to the bank for {}.",
                    turn,
                    dollars,
                    msg
                );

                let current_player = &mut self.players[turn as usize];
                let (result, mut logs) = current_player.pay(&mut self.board, dollars);
                self.logs.append(&mut logs);

                // If the player cannot pay, their property will be returned to the bank.
                if let Err(_) = result {
                    game_log!(
                        self,
                        "[PLAYER{}] All of the properties are returned to the bank.",
                        turn
                    );

                    // Reset the properties.
                    let player_places = self
                        .board
                        .places
                        .iter_mut()
                        .filter(|place| place.get_owner() == Some(turn));
                    for place in player_places {
                        place.set_owner(None);
                        place.set_mortgaged(false);
                        if place.is_estate() {
                            place.set_num_houses(0);
                        }
                    }
                }
            }
            EventKind::PayToOther(msg, receiver, dollars) => {
                game_log!(
                    self,
                    "[PLAYER{}] Pays ${} to PLAYER{} for {}.",
                    turn,
                    dollars,
                    receiver,
                    msg
                );

                let current_player = &mut self.players[turn as usize];
                let (result, mut logs) = current_player.pay(&mut self.board, dollars);
                self.logs.append(&mut logs);

                match result {
                    Ok(_) => {
                        let receiver = self.get_player_mut(receiver);
                        receiver.money += dollars;
                    }
                    Err(money) => {
                        // When the player cannot pay, their properties will be moved to the creditor.
                        let receiver = self.get_player_mut(receiver);
                        receiver.money += money;
                        let receiver_id = receiver.player_id;

                        game_log!(
                            self,
                            "[PLAYER{}] Inherits properties of PLAYER{}.",
                            receiver_id,
                            turn
                        );

                        let player_places = self
                            .board
                            .places
                            .iter_mut()
                            .filter(|place| place.get_owner() == Some(turn));
                        for place in player_places {
                            place.set_owner(Some(receiver_id));
                        }
                    }
                }
            }
            EventKind::Reward(msg, dollars) => {
                game_log!(self, "[PLAYER{}] Gains ${} for {}.", turn, dollars, msg);

                let current_player = self.get_current_player_mut();
                current_player.money += dollars;
            }
            EventKind::Move(msg, mut place) => {
                game_log!(self, "[PLAYER{}] Needs to move for {}.", turn, msg);

                let current_player = self.get_current_player_mut();
                let position = current_player.position;
                if place < position {
                    place += self.board.places.len();
                }
                self.move_player(place - position)
            }
            EventKind::GivePlace(place, dollars) => {
                let place_name = self.board.places[place].get_place_name();

                game_log!(
                    self,
                    "[PLAYER{}] Buys {} for ${}.",
                    turn,
                    place_name,
                    dollars
                );

                self.invoke_event(EventKind::PayToBank(place_name, dollars));

                let current_player = self.get_current_player();
                if current_player.state != PlayerState::Bankrupted {
                    self.board.places[place].set_owner(Some(turn));
                }
            }
            EventKind::GetJailed => {
                game_log!(self, "[PLAYER{}] Gets jailed.", turn);

                let current_player = self.get_current_player_mut();
                current_player.state = PlayerState::InJail(0);
                current_player.position = JAIL_POSITION;
            }
        }
    }

    ///
    /// Has the player move.
    ///
    pub fn move_player(&mut self, count: usize) {
        let current_player = self.get_current_player_mut();
        let previous_position = current_player.position;
        let previous_position_name = self.board.places[previous_position].get_place_name();

        // If the player passes GO, they receive $200.
        let mut new_position = previous_position + count;
        if new_position >= self.board.places.len() {
            new_position -= self.board.places.len();

            self.invoke_event(EventKind::Reward("passing GO", 200))
        }

        let current_player = self.get_current_player_mut();
        current_player.position = new_position;
        let new_position_name = self.board.places[new_position].get_place_name();

        self.invoke_event(EventKind::None(&format!(
            "Moves from {} to {}.",
            previous_position_name, new_position_name
        )));

        self.invoke_event(self.board.places[new_position].get_action(self.turn, &self.board));
    }

    ///
    /// Emulates a turn.
    ///
    /// This method takes account of the effect of the same number.
    ///
    pub fn spend_one_turn(&mut self) {
        if self.count_active_players() > 1 {
            self.spend_one_turn_internal(0);
        }

        // Since the number of active players can be changed through moving,
        // check it again here.
        if self.count_active_players() > 1 {
            for player in &mut self.players {
                player.invest(&mut self.board);
            }

            self.turn += 1;
            if self.turn >= self.players.len() {
                self.turn -= self.players.len();
            }
        }
    }

    ///
    /// Emulates a turn.
    ///
    /// **Do not call this method directly, unless you are in `spend_one_turn`.**
    ///
    fn spend_one_turn_internal(&mut self, recursion: u32) {
        if recursion == 3 {
            self.invoke_event(EventKind::None("Breaks the speed limit."));
            self.invoke_event(EventKind::GetJailed);
            return;
        }

        let turn = self.turn;
        let mut current_player = self.get_current_player_mut();

        assert_eq!(turn, current_player.player_id);

        match current_player.state {
            PlayerState::None => match DiceRolling::roll() {
                DiceResult::Same(result) => {
                    self.move_player(result as usize);

                    let current_player = self.get_current_player_mut();
                    if let PlayerState::InJail(_) = current_player.state {
                    } else {
                        self.spend_one_turn_internal(recursion + 1);
                    }
                }
                DiceResult::Different(result) => {
                    self.move_player(result as usize);
                }
            },
            PlayerState::Bankrupted => {
                self.invoke_event(EventKind::None("Already bankrupted."));
            }
            PlayerState::InJail(jail_count) => match DiceRolling::roll() {
                DiceResult::Same(result) => {
                    current_player.state = PlayerState::None;
                    self.invoke_event(EventKind::None("Leaves the jail."));
                    self.move_player(result as usize);
                }
                DiceResult::Different(result) => {
                    if jail_count == 2 {
                        current_player.state = PlayerState::None;
                        self.invoke_event(EventKind::None("Completes their term of jail."));
                        self.invoke_event(EventKind::PayToBank("Jail fee", 50));
                        self.move_player(result as usize);
                    } else {
                        current_player.state = PlayerState::InJail(jail_count + 1);
                        self.invoke_event(EventKind::None("Stays in the jail."));
                    }
                }
            },
        }
    }
}

///
/// Represents a board.
///
/// This can be seen as a set of places with some useful functions.
///
pub struct Board {
    pub places: Vec<Box<dyn BoardPlace>>,
}

impl Board {
    ///
    /// Generates a board.
    ///
    pub fn new() -> Self {
        Board {
            places: get_place_list(),
        }
    }

    ///
    /// Gets the sum of the numbers of houses which stand on the designated color.
    ///
    pub fn get_houses_num_by_color(&self, color: BoardColor) -> Option<u8> {
        let mut num = 0;
        for place in &self.places {
            if place.get_color() == color {
                num += place.get_num_houses()?
            }
        }
        Some(num)
    }

    ///
    /// Gets an iterator of places with the designated color.
    ///
    pub fn gets_by_color(&self, color: BoardColor) -> impl Iterator<Item = &Box<dyn BoardPlace>> {
        self.places
            .iter()
            .filter(move |place| place.get_color() == color)
    }

    ///
    /// Gets a mutable iterator of places with the designated color.
    ///
    pub fn gets_by_color_mut(
        &mut self,
        color: BoardColor,
    ) -> impl Iterator<Item = &mut Box<dyn BoardPlace>> {
        self.places
            .iter_mut()
            .filter(move |place| place.get_color() == color)
    }

    ///
    /// Gets a monopolizer of the color if exists.
    ///
    pub fn get_monopolizer(&self, color: BoardColor) -> Option<usize> {
        let mut owners = self.places.iter().filter_map(|place| {
            if place.get_color() == color {
                Some(place.get_owner())
            } else {
                None
            }
        });
        if let Some(possible_owner) = owners.next().unwrap() {
            if owners.all(|owner| owner == Some(possible_owner)) {
                Some(possible_owner)
            } else {
                None
            }
        } else {
            None
        }
    }

    ///
    /// Gets whether places of the color is monopolized by someone.
    ///
    pub fn is_monopolized(&self, color: BoardColor) -> bool {
        self.get_monopolizer(color).is_some()
    }

    ///
    /// Validates the board in terms of houses.
    ///
    /// After building or destructing houses, you should call this to prevent unintentional behaviors.
    ///
    pub fn validate_houses(&self) {
        for color in BoardColor::get_estate_colors() {
            let places = self
                .places
                .iter()
                .filter(|place| place.get_color() == color)
                .collect::<Vec<_>>();
            let houses = places
                .iter()
                .map(|place| place.get_num_houses().unwrap())
                .collect::<Vec<_>>();

            // If there is at least one house, it infers that these areas are monopolized and that all of them are not mortgaged.
            if houses.iter().sum::<u8>() > 0 {
                assert!(self.is_monopolized(color.clone()));
                assert!(places.iter().all(|&place| !place.is_mortgaged()));
            }

            // Due to the rule of building (or sometimes destructing) a house, the houses should be built "flatly".
            assert!(houses.iter().max().unwrap() - houses.iter().min().unwrap() <= 1);
        }
    }

    ///
    /// Gets the most expensive
    ///
    pub fn get_most_expensive(&self, excluding: usize) -> u32 {
        let mut most_expensive = 0u32;
        for place in &self.places {
            if place.get_owner() != Some(excluding) {
                let rent = place.get_rent(self).unwrap_or(0);

                if rent > most_expensive {
                    most_expensive = rent;
                }
            }
        }
        most_expensive
    }
}
