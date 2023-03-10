use crate::board::Board;
use crate::places::{BoardAction, BoardColor, BoardPlace};

pub struct CommunityChest {
    id: usize,
}

impl BoardPlace for CommunityChest {
    fn info(&self) -> String {
        format!("{}", "Community Chest")
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_place_name(&self) -> &'static str {
        "Community Chest"
    }

    fn get_action<'a>(&self, _: &Board) -> BoardAction<'a> {
        BoardAction::None("Lands Community Chest.")
    }

    fn get_owner(&self) -> Option<usize> {
        None
    }

    fn set_owner(&mut self, _: usize) {
        panic!("You cannot set the owner of Community Chest.");
    }

    fn get_num_houses(&self) -> Option<u8> {
        None
    }

    fn set_num_houses(&mut self, _: u8) {
        panic!("You cannot build a house on Community Chest.");
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

impl CommunityChest {
    pub fn new(id: usize) -> Box<dyn BoardPlace> {
        Box::new(CommunityChest { id })
    }
}
