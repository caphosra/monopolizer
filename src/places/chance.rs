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

    fn get_action<'a>(&self, _: usize, _: &Board) -> BoardAction<'a> {
        BoardAction::None("Lands Chance.")
    }

    fn get_owner(&self) -> Option<usize> {
        None
    }

    fn set_owner(&mut self, _: Option<usize>) {
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

    fn get_price(&self) -> u32 {
        panic!("The price of Chance is undefined.");
    }

    fn get_color(&self) -> BoardColor {
        BoardColor::None
    }

    fn is_mortgaged(&self) -> bool {
        false
    }

    fn set_mortgaged(&mut self, _: bool) -> u32 {
        panic!("You cannot mortgage Chance.");
    }
}

impl Chance {
    pub fn new(id: usize) -> Box<dyn BoardPlace> {
        Box::new(Chance { id })
    }
}
