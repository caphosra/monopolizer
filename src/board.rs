use crate::actions::BoardAction;
use crate::dice_rolling::{DiceResult, DiceRolling};
use crate::places::{get_place_list, BoardColor, BoardPlace};
use crate::player::{Player, PlayerState};
use crate::strategy::ExpensiveHousesProtectionStrategy;

const JAIL_POSITION: usize = 10;

pub struct MonopolyGame {
    pub players: Vec<Player>,
    pub board: Board,
    pub turn: usize,
    pub logs: Vec<String>,
}

impl MonopolyGame {
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

    pub fn log(&mut self, log: String) {
        self.logs.push(log);
    }

    pub fn get_mut_player(&mut self, id: usize) -> &mut Player {
        assert!(id < self.players.len());

        &mut self.players[id]
    }

    pub fn get_player(&self, id: usize) -> &Player {
        assert!(id < self.players.len());

        &self.players[id]
    }

    pub fn get_mut_current_player(&mut self) -> &mut Player {
        self.get_mut_player(self.turn as usize)
    }

    pub fn get_current_player(&self) -> &Player {
        self.get_player(self.turn as usize)
    }

    pub fn exec_action(&mut self, action: BoardAction) {
        let turn = self.turn;
        match action {
            BoardAction::None(msg) => {
                self.log(format!("[PLAYER{}] {}", turn, msg));
            }
            BoardAction::PayToBank(msg, dollars) => {
                self.log(format!(
                    "[PLAYER{}] Pays ${} to the bank for {}.",
                    turn, dollars, msg
                ));

                let current_player = &mut self.players[turn as usize];
                let (result, mut logs) = current_player.pay(&mut self.board, dollars);
                self.logs.append(&mut logs);

                if let Err(_) = result {
                    self.log(format!(
                        "[PLAYER{}] All of the properties are returned to the bank.",
                        turn
                    ));

                    let player_places = self
                        .board
                        .places
                        .iter_mut()
                        .filter(|place| place.get_owner() == Some(turn));
                    for place in player_places {
                        place.set_owner(None);
                        place.set_mortgaged(false);
                        place.set_num_houses(0);
                    }
                }
            }
            BoardAction::PayToOther(msg, receiver, dollars) => {
                self.log(format!(
                    "[PLAYER{}] Pays ${} to PLAYER{} for {}.",
                    turn, dollars, receiver, msg
                ));

                let current_player = &mut self.players[turn as usize];
                let (result, mut logs) = current_player.pay(&mut self.board, dollars);
                self.logs.append(&mut logs);

                match result {
                    Ok(_) => {
                        let receiver = self.get_mut_player(receiver);
                        receiver.money += dollars;
                    }
                    Err(money) => {
                        let receiver = self.get_mut_player(receiver);
                        receiver.money += money;
                        let receiver_id = receiver.player_id;

                        self.log(format!(
                            "[PLAYER{}] Inherits properties of PLAYER{}.",
                            receiver_id, turn
                        ));

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
            BoardAction::Reward(msg, dollars) => {
                self.logs
                    .push(format!("[PLAYER{}] Gains ${} for {}.", turn, dollars, msg));

                let current_player = self.get_mut_current_player();
                current_player.money += dollars;
            }
            BoardAction::Move(msg, mut place) => {
                self.logs
                    .push(format!("[PLAYER{}] Needs to move for {}.", turn, msg));

                let current_player = self.get_mut_current_player();
                let position = current_player.position;
                if place < position {
                    place += self.board.places.len();
                }
                self.move_player(place - position)
            }
            BoardAction::GivePlace(place, dollars) => {
                let place_name = self.board.places[place].get_place_name();

                self.log(format!(
                    "[PLAYER{}] Buys {} for ${}.",
                    turn, place_name, dollars
                ));

                self.exec_action(BoardAction::PayToBank(place_name, dollars));

                let current_player = self.get_current_player();
                if current_player.state != PlayerState::Bankrupted {
                    self.board.places[place].set_owner(Some(turn));
                }
            }
            BoardAction::GetJailed => {
                self.log(format!("[PLAYER{}] Gets jailed.", turn));

                let current_player = self.get_mut_current_player();
                current_player.state = PlayerState::InJail(0);
                current_player.position = JAIL_POSITION;
            }
        }
    }

    pub fn move_player(&mut self, count: usize) {
        let current_player = self.get_mut_current_player();
        let previous_position = current_player.position;
        let previous_position_name = self.board.places[previous_position].get_place_name();

        let mut new_position = previous_position + count;
        if new_position >= self.board.places.len() {
            new_position -= self.board.places.len();

            self.exec_action(BoardAction::Reward("passing GO", 200))
        }

        let current_player = self.get_mut_current_player();
        current_player.position = new_position;
        let new_position_name = self.board.places[new_position].get_place_name();

        self.exec_action(BoardAction::None(&format!(
            "Moves from {} to {}.",
            previous_position_name, new_position_name
        )));

        self.exec_action(self.board.places[new_position].get_action(self.turn, &self.board));
    }

    pub fn spend_one_turn(&mut self) {
        self.spend_one_turn_internal(0);

        for player in &mut self.players {
            player.invest(&mut self.board);
        }

        self.turn += 1;
        if self.turn >= self.players.len() {
            self.turn -= self.players.len();
        }
    }

    fn spend_one_turn_internal(&mut self, recursion: u32) {
        if recursion == 3 {
            self.exec_action(BoardAction::None("Breaks the speed limit."));
            self.exec_action(BoardAction::GetJailed);
            return;
        }

        let turn = self.turn;
        let mut current_player = self.get_mut_current_player();

        assert_eq!(turn, current_player.player_id);

        match current_player.state {
            PlayerState::None => match DiceRolling::roll() {
                DiceResult::Same(result) => {
                    self.move_player(result as usize);

                    let current_player = self.get_mut_current_player();
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
                self.exec_action(BoardAction::None("Already bankrupted."));
            }
            PlayerState::InJail(jail_count) => match DiceRolling::roll() {
                DiceResult::Same(result) => {
                    current_player.state = PlayerState::None;
                    self.exec_action(BoardAction::None("Leaves the jail."));
                    self.move_player(result as usize);
                }
                DiceResult::Different(result) => {
                    if jail_count == 2 {
                        current_player.state = PlayerState::None;
                        self.exec_action(BoardAction::None("Completes their term of jail."));
                        self.exec_action(BoardAction::PayToBank("Jail fee", 50));
                        self.move_player(result as usize);
                    } else {
                        current_player.state = PlayerState::InJail(jail_count + 1);
                        self.exec_action(BoardAction::None("Stays in the jail."));
                    }
                }
            },
        }
    }

    pub fn get_players_on_place(&self, place_id: usize) -> Vec<&Player> {
        self.players
            .iter()
            .filter(|player| player.position == place_id)
            .collect()
    }
}

pub struct Board {
    pub places: Vec<Box<dyn BoardPlace>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            places: get_place_list(),
        }
    }

    pub fn get_houses_num_by_color(&self, color: BoardColor) -> Option<u8> {
        let mut num = 0;
        for place in &self.places {
            if place.get_color() == color {
                num += place.get_num_houses()?
            }
        }
        Some(num)
    }

    pub fn gets_by_color(&self, color: BoardColor) -> impl Iterator<Item = &Box<dyn BoardPlace>> {
        self.places
            .iter()
            .filter(move |place| place.get_color() == color)
    }

    pub fn gets_by_color_mut(
        &mut self,
        color: BoardColor,
    ) -> impl Iterator<Item = &mut Box<dyn BoardPlace>> {
        self.places
            .iter_mut()
            .filter(move |place| place.get_color() == color)
    }

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
                assert!(
                    self.is_monopolized(color.clone())
                        && places.iter().all(|place| !place.is_mortgaged())
                );
            }

            // Due to the rule of building (or sometimes destructing) a house, the houses should be built "flatly".
            assert!(houses.iter().max().unwrap() - houses.iter().min().unwrap() <= 1);
        }
    }

    pub fn get_most_expensive(&self, excluding: usize) -> u32 {
        let mut most_expensive = 0u32;
        for place in &self.places {
            match place.get_action(excluding, self) {
                BoardAction::PayToBank(_, money) => {
                    if money > most_expensive {
                        most_expensive = money;
                    }
                }
                BoardAction::PayToOther(_, _, money) => {
                    if money > most_expensive {
                        most_expensive = money;
                    }
                }
                _ => {}
            };
        }
        most_expensive
    }
}
