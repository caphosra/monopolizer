use crate::board::Board;
use crate::strategy::PlayerStrategy;

///
/// Represents the state of a player.
///
#[derive(PartialEq, Eq)]
pub enum PlayerState {
    None,
    Bankrupted,
    InJail(u8),
}

///
/// Holds the information of a player.
///
pub struct Player {
    pub player_id: usize,
    pub money: u32,
    pub state: PlayerState,
    pub position: usize,
    strategy: Box<dyn PlayerStrategy + Send>,
}

impl Player {
    ///
    /// Generates a player with designated strategy.
    ///
    pub fn new(player_id: usize, strategy: Box<dyn PlayerStrategy + Send>) -> Self {
        Player {
            player_id,
            money: 1500,
            state: PlayerState::None,
            position: 0,
            strategy,
        }
    }

    ///
    /// Makes the player pay the rent.
    ///
    pub fn pay(&mut self, board: &mut Board, dollars: u32) -> (Result<(), u32>, Vec<String>) {
        match self.strategy.raise(
            dollars,
            board,
            self.player_id,
            &mut self.money,
            &self.state,
            self.position,
        ) {
            Ok(_) => (
                Ok(()),
                vec![format!(
                    "[PLAYER{}] Money: ${} -> ${}",
                    self.player_id,
                    self.money + dollars,
                    self.money
                )],
            ),
            Err(money) => {
                assert!(money < dollars);

                self.money = 0;
                self.state = PlayerState::Bankrupted;

                (
                    Err(money),
                    vec![format!("[PLAYER{}] Bankrupted", self.player_id)],
                )
            }
        }
    }

    ///
    /// Makes the player do investment within the budget.
    ///
    pub fn invest(&mut self, board: &mut Board) {
        self.strategy.invest(
            board,
            self.player_id,
            &mut self.money,
            &self.state,
            self.position,
        );
    }
}
