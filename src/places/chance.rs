use crate::board::Board;
use crate::places::{BoardAction, BoardColor, BoardPlace};

pub struct Chance {
    id: usize,
}

impl BoardPlace for Chance {
    fn info(&self) -> String {
        format!("{}", "Chance")
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_place_name(&self) -> &'static str {
        "Chance"
    }

    fn get_action<'a>(&self, _: &Board) -> BoardAction<'a> {
        BoardAction::None("Lands Chance.")
    }

    fn get_owner(&self) -> Option<usize> {
        None
    }

    fn set_owner(&mut self, _: usize) {
        panic!("You cannot set the owner of Chance.");
    }

    fn get_num_houses(&self) -> Option<u8> {
        None
    }

    fn set_num_houses(&mut self, _: u8) {
        panic!("You cannot build a house on Chance.");
    }

    fn get_price_of_house(&self) -> Option<u32> {
        None
    }

    fn get_color(&self) -> BoardColor {
        BoardColor::None
    }

    fn is_monopolized(&self, _: &Board) -> bool {
        false
    }

    fn is_mortgaged(&self) -> bool {
        false
    }
}

impl Chance {
    pub fn new(id: usize) -> Box<dyn BoardPlace> {
        Box::new(Chance { id })
    }
}