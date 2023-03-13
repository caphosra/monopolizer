use crate::board::Board;
use crate::dice_rolling::DiceRolling;
use crate::places::{BoardAction, BoardColor, BoardPlace};

pub struct Utilities {
    id: usize,
    name: &'static str,
    owner: Option<usize>,
    mortgaged: bool,
}

impl BoardPlace for Utilities {
    fn info(&self) -> String {
        if let Some(owner) = self.owner {
            format!("{} owner:{}", self.name, owner)
        } else {
            format!("{}", self.name)
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_place_name(&self) -> &'static str {
        self.name
    }

    fn get_action<'a>(&self, turn: usize, board: &Board) -> BoardAction<'a> {
        if let Some(owner) = self.owner {
            if owner == turn {
                BoardAction::None("Lands their place.")
            } else {
                if self.mortgaged {
                    BoardAction::None("The place is mortgaged.")
                } else {
                    let rent = match self.get_own_num(board) {
                        1 => DiceRolling::roll().get_num() * 4,
                        2 => DiceRolling::roll().get_num() * 10,
                        _ => panic!("The number of utilities is invalid."),
                    };
                    BoardAction::PayToOther(self.get_place_name(), owner, rent)
                }
            }
        } else {
            BoardAction::GivePlace(self.id, 150)
        }
    }

    fn get_owner(&self) -> Option<usize> {
        self.owner
    }

    fn set_owner(&mut self, owner: usize) {
        self.owner = Some(owner);
    }

    fn get_num_houses(&self) -> Option<u8> {
        None
    }

    fn set_num_houses(&mut self, _: u8) {
        panic!("You cannot build a house on Utilities.");
    }

    fn get_price_of_house(&self) -> Option<u32> {
        None
    }

    fn get_color(&self) -> BoardColor {
        BoardColor::Utilities
    }

    fn is_mortgaged(&self) -> bool {
        self.mortgaged
    }

    fn set_mortgaged(&mut self, mortgaged: bool) -> u32 {
        self.mortgaged = mortgaged;
        75
    }
}

impl Utilities {
    pub fn new(id: usize, name: &'static str) -> Box<dyn BoardPlace> {
        Box::new(Utilities {
            id,
            name,
            owner: None,
            mortgaged: false,
        })
    }

    fn get_own_num(&self, board: &Board) -> u32 {
        board
            .places
            .iter()
            .filter(|place| {
                place.get_color() == BoardColor::Utilities
                    && place.get_owner() == self.owner
                    && place.is_mortgaged() == false
            })
            .count() as u32
    }
}
