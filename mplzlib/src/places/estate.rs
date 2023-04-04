use crate::board::Board;
use crate::events::EventKind;
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

    fn get_action<'a>(&self, turn: usize, board: &Board) -> EventKind<'a> {
        if let Some(owner) = self.owner {
            if owner == turn {
                EventKind::None("Lands their place.")
            } else {
                if self.mortgaged {
                    EventKind::None("The place is mortgaged.")
                } else {
                    let rent = self.get_rent(board);
                    EventKind::PayToOther(self.get_place_name(), owner, rent)
                }
            }
        } else {
            EventKind::GivePlace(self.id, self.price)
        }
    }

    fn get_owner(&self) -> Option<usize> {
        self.owner
    }

    fn set_owner(&mut self, owner: Option<usize>) {
        self.owner = owner;
    }

    fn get_num_houses(&self) -> Option<u8> {
        Some(self.houses)
    }

    fn get_price(&self) -> u32 {
        self.price
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

    fn is_mortgaged(&self) -> bool {
        self.mortgaged
    }

    fn set_mortgaged(&mut self, mortgaged: bool) -> u32 {
        self.mortgaged = mortgaged;
        self.price / 2
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
        let color = self.get_color();
        let rent_id = board.is_monopolized(color) as u8 + self.houses;
        self.rent[rent_id as usize]
    }
}
