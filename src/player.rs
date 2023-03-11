#[derive(PartialEq, Eq)]
pub enum PlayerState {
    None,
    Bankrupted,
    InJail(u32),
}

pub struct Player {
    pub player_id: usize,
    pub money: u32,
    pub state: PlayerState,
    pub position: usize,
}

impl Player {
    pub fn new(player_id: usize) -> Self {
        Player {
            player_id,
            money: 1500,
            state: PlayerState::None,
            position: 0,
        }
    }

    pub fn pay(&mut self, dollars: u32) -> Vec<String> {
        if self.money < dollars {
            self.money = 0;

            self.state = PlayerState::Bankrupted;

            vec![format!("[PLAYER{}] Bankrupted", self.player_id,)]
        } else {
            self.money -= dollars;

            vec![format!(
                "[PLAYER{}] Money: ${} -> ${}",
                self.player_id,
                self.money + dollars,
                self.money
            )]
        }
    }
}
