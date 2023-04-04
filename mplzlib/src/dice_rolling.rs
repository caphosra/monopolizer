use rand::{thread_rng, Rng};

///
/// Holds a result of dice-rolling.
///
pub enum DiceResult {
    Same(u32),
    Different(u32),
}

impl DiceResult {
    ///
    /// Unwraps the result.
    ///
    pub fn unwrap(self) -> u32 {
        match self {
            DiceResult::Same(result) => result,
            DiceResult::Different(result) => result,
        }
    }
}

///
/// Has a function of rolling dices.
///
pub struct DiceRolling;

impl DiceRolling {
    ///
    /// Rolls dices.
    ///
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
