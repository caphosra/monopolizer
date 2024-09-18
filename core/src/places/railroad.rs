use crate::board::Board;
use crate::places::{BoardColor, BoardPlace, EventKind};

pub struct Railroad {
    id: usize,
    name: &'static str,
    owner: Option<usize>,
    mortgaged: bool,
}

impl BoardPlace for Railroad {
    fn info(&self) -> String {
        if let Some(owner) = self.owner {
            format!("{} owner:{}", self.name, owner)
        } else {
            self.name.to_string()
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_place_name(&self) -> &'static str {
        self.name
    }

    fn get_action<'a>(&self, turn: usize, board: &Board) -> EventKind<'a> {
        if let Some(owner) = self.owner {
            if owner == turn {
                EventKind::None("Lands their place.")
            } else if self.mortgaged {
                EventKind::None("The place is mortgaged.")
            } else {
                let rent = match self.get_own_num(board) {
                    1 => 25,
                    2 => 50,
                    3 => 100,
                    4 => 200,
                    _ => panic!("The number of railroads is invalid."),
                };
                EventKind::PayToOther(self.get_place_name(), owner, rent)
            }
        } else {
            EventKind::GivePlace(self.id, 200)
        }
    }

    fn get_owner(&self) -> Option<usize> {
        self.owner
    }

    fn set_owner(&mut self, owner: Option<usize>) {
        self.owner = owner;
    }

    fn get_num_houses(&self) -> Option<u8> {
        None
    }

    fn set_num_houses(&mut self, _: u8) {
        panic!("You cannot build a house on a railroad.");
    }

    fn get_price_of_house(&self) -> Option<u32> {
        None
    }

    fn get_price(&self) -> u32 {
        200
    }

    fn get_color(&self) -> BoardColor {
        BoardColor::Railroad
    }

    fn is_mortgaged(&self) -> bool {
        self.mortgaged
    }

    fn set_mortgaged(&mut self, mortgaged: bool) -> u32 {
        self.mortgaged = mortgaged;
        100
    }
}

impl Railroad {
    pub fn new_boxed(id: usize, name: &'static str) -> Box<dyn BoardPlace + Send> {
        Box::new(Railroad {
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
                place.get_color() == BoardColor::Railroad
                    && place.get_owner() == self.owner
                    && !place.is_mortgaged()
            })
            .count() as u32
    }
}
