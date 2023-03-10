use crate::actions::BoardAction;
use crate::board::Board;
use crate::places::{BoardColor, BoardPlace};

pub struct Estate {
    id: usize,
    color: BoardColor,
    name: &'static str,
    owner: Option<usize>,
    houses: u8,
    price: u32,
    house_price: u32,
    rent: Vec<u32>,
    mortgaged: bool,
}

impl BoardPlace for Estate {
    fn info(&self) -> String {
        if let Some(owner) = self.owner {
            format!(
                "{} [{}] owner:{} mortgaged:{} houses:{}",
                self.name, self.color, owner, self.mortgaged, self.houses
            )
        } else {
            format!("{} [{}]", self.name, self.color)
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_place_name(&self) -> &'static str {
        self.name
    }

    fn get_action<'a>(&self, board: &Board) -> BoardAction<'a> {
        if let Some(owner) = self.owner {
            if owner == board.turn {
                BoardAction::None("Lands their place.")
            } else {
                if self.mortgaged {
                    BoardAction::None("The place is mortgaged.")
                } else {
                    let rent = self.get_rent(board);
                    BoardAction::PayToOther(self.get_place_name(), owner, rent)
                }
            }
        } else {
            BoardAction::GivePlace(self.id, self.price)
        }
    }

    fn get_owner(&self) -> Option<usize> {
        self.owner
    }

    fn set_owner(&mut self, owner: usize) {
        self.owner = Some(owner);
    }

    fn get_num_houses(&self) -> Option<u8> {
        Some(self.houses)
    }

    fn set_num_houses(&mut self, num: u8) {
        self.houses = num;
    }

    fn get_price_of_house(&self) -> Option<u32> {
        Some(self.house_price)
    }

    fn get_color(&self) -> BoardColor {
        self.color.clone()
    }

    fn is_monopolized(&self, board: &Board) -> bool {
        if let Some(owner) = self.get_owner() {
            board
                .places
                .iter()
                .filter(|place| place.get_color() == self.color)
                .all(|place| place.get_owner() == Some(owner))
        } else {
            false
        }
    }

    fn is_mortgaged(&self) -> bool {
        self.mortgaged
    }
}

impl Estate {
    pub fn new(
        id: usize,
        color: BoardColor,
        name: &'static str,
        price: u32,
        house_price: u32,
        rent: Vec<u32>,
    ) -> Box<dyn BoardPlace> {
        assert_eq!(rent.len(), 7);

        Box::new(Estate {
            id,
            color,
            name,
            owner: None,
            houses: 0,
            price,
            house_price,
            rent,
            mortgaged: false,
        })
    }

    pub fn get_rent(&self, board: &Board) -> u32 {
        let rent_id = self.is_monopolized(board) as u8 + self.houses;
        self.rent[rent_id as usize]
    }
}
