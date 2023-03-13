use crate::board::Board;
use crate::places::{BoardAction, BoardColor, BoardPlace};

pub struct IncomeTax {
    id: usize,
}

impl BoardPlace for IncomeTax {
    fn info(&self) -> String {
        format!("{}", "Income Tax")
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_place_name(&self) -> &'static str {
        "Income Tax"
    }

    fn get_action<'a>(&self, _: usize, _: &Board) -> BoardAction<'a> {
        BoardAction::PayToBank("Income Tax", 200)
    }

    fn get_owner(&self) -> Option<usize> {
        None
    }

    fn set_owner(&mut self, _: Option<usize>) {
        panic!("You cannot set the owner of Income Tax.");
    }

    fn get_num_houses(&self) -> Option<u8> {
        None
    }

    fn set_num_houses(&mut self, _: u8) {
        panic!("You cannot build a house on Income Tax.");
    }

    fn get_price_of_house(&self) -> Option<u32> {
        None
    }

    fn get_color(&self) -> BoardColor {
        BoardColor::None
    }

    fn is_mortgaged(&self) -> bool {
        false
    }

    fn set_mortgaged(&mut self, _: bool) -> u32 {
        panic!("You cannot mortgage Income Tax.");
    }
}

impl IncomeTax {
    pub fn new(id: usize) -> Box<dyn BoardPlace> {
        Box::new(IncomeTax { id })
    }
}
