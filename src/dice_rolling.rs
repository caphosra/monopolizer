use rand::{thread_rng, Rng};

pub enum DiceResult {
    Same(u32),
    Different(u32),
}

impl DiceResult {
    pub fn get_num(self) -> u32 {
        match self {
            DiceResult::Same(result) => result,
            DiceResult::Different(result) => result,
        }
    }
}

pub struct DiceRolling;

impl DiceRolling {
    pub fn roll() -> DiceResult {
        let mut thread = thread_rng();
        let first = thread.gen_range(1..(6 + 1));
        let second = thread.gen_range(1..(6 + 1));
        if first == second {
            DiceResult::Same(first + second)
        } else {
            DiceResult::Different(first + second)
        }
    }
}
