use crate::board::Board;
use crate::places::{BoardColor, BoardPlace, EventKind};

pub struct Nothing {
    id: usize,
    name: &'static str,
}

impl BoardPlace for Nothing {
    fn info(&self) -> String {
        format!("{}", self.name)
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_place_name(&self) -> &'static str {
        self.name
    }

    fn get_action<'a>(&self, _: usize, _: &Board) -> EventKind<'a> {
        EventKind::None(self.name)
    }

    fn get_owner(&self) -> Option<usize> {
        None
    }

    fn set_owner(&mut self, _: Option<usize>) {
        panic!("You cannot set the owner.");
    }

    fn get_num_houses(&self) -> Option<u8> {
        None
    }

    fn set_num_houses(&mut self, _: u8) {
        panic!("You cannot build a house.");
    }

    fn get_price_of_house(&self) -> Option<u32> {
        None
    }

    fn get_price(&self) -> u32 {
        panic!("The price is undefined.");
    }

    fn get_color(&self) -> BoardColor {
        BoardColor::None
    }

    fn is_mortgaged(&self) -> bool {
        false
    }

    fn set_mortgaged(&mut self, _: bool) -> u32 {
        panic!("You cannot mortgage it.");
    }
}

impl Nothing {
    pub fn new(id: usize, name: &'static str) -> Box<dyn BoardPlace> {
        Box::new(Nothing { id, name })
    }
}
