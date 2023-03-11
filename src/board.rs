use crate::actions::BoardAction;
use crate::dice_rolling::{DiceResult, DiceRolling};
use crate::places::{get_place_list, BoardPlace};
use crate::player::{Player, PlayerState};

const JAIL_POSITION: usize = 10;

pub struct Board {
    pub players: Vec<Player>,
    pub places: Vec<Box<dyn BoardPlace>>,
    pub turn: usize,
    logs: Vec<String>,
}

impl Board {
    pub fn new(players: Vec<Player>) -> Self {
        Board {
            players,
            places: get_place_list(),
            turn: 0,
            logs: vec![],
        }
    }

    pub fn get_logs(&self) -> &Vec<String> {
        &self.logs
    }

    pub fn log(&mut self, log: String) {
        self.logs.push(log);
    }

    pub fn info(&self) -> String {
        self.places
            .iter()
            .map(|place| place.info())
            .collect::<Vec<String>>()
            .join("\n")
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

                let current_player = self.get_mut_current_player();
                let mut logs = current_player.pay(dollars);
                self.logs.append(&mut logs);
            }
            BoardAction::PayToOther(msg, receiver, dollars) => {
                self.log(format!(
                    "[PLAYER{}] Pays ${} to PLAYER{} for {}.",
                    turn, dollars, receiver, msg
                ));

                let current_player = self.get_mut_current_player();
                let mut logs = current_player.pay(dollars);
                self.logs.append(&mut logs);

                let receiver = self.get_mut_player(receiver);
                receiver.money += dollars;
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
                    place += self.places.len();
                }
                self.move_player(place - position)
            }
            BoardAction::GivePlace(place, dollars) => {
                let place_name = self.places[place].get_place_name();

                self.log(format!(
                    "[PLAYER{}] Buys {} for ${}.",
                    turn, place_name, dollars
                ));

                self.exec_action(BoardAction::PayToBank(place_name, dollars));

                let current_player = self.get_current_player();
                if PlayerState::Bankrupted != current_player.state {
                    self.places[place].set_owner(turn);
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
        let previous_position_name = self.places[previous_position].get_place_name();

        let mut new_position = previous_position + count;
        if new_position >= self.places.len() {
            new_position -= self.places.len();

            self.exec_action(BoardAction::Reward("passing GO", 200))
        }

        let current_player = self.get_mut_current_player();
        current_player.position = new_position;
        let new_position_name = self.places[new_position].get_place_name();

        self.exec_action(BoardAction::None(&format!(
            "Moves from {} to {}.",
            previous_position_name, new_position_name
        )));

        self.exec_action(self.places[new_position].get_action(self));
    }

    pub fn spend_one_turn(&mut self) {
        self.spend_one_turn_internal(0);
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
