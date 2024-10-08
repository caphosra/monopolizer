pub mod chance;
pub mod community_chest;
pub mod estate;
pub mod go_to_jail;
pub mod income_tax;
pub mod luxury_tax;
pub mod nothing;
pub mod railroad;
pub mod utilities;

use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::board::Board;
use crate::events::EventKind;
use crate::places::chance::Chance;
use crate::places::community_chest::CommunityChest;
use crate::places::estate::Estate;
use crate::places::go_to_jail::GoToJail;
use crate::places::income_tax::IncomeTax;
use crate::places::luxury_tax::LuxuryTax;
use crate::places::nothing::Nothing;
use crate::places::railroad::Railroad;
use crate::places::utilities::Utilities;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum BoardColor {
    None,
    Railroad,
    Utilities,
    Brown,
    LightBlue,
    LightPurple,
    Orange,
    Red,
    Yellow,
    Green,
    Blue,
}

impl Display for BoardColor {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{:?}", self)
    }
}

impl BoardColor {
    pub fn get_estate_colors() -> Vec<BoardColor> {
        vec![
            BoardColor::Brown,
            BoardColor::LightBlue,
            BoardColor::LightPurple,
            BoardColor::Orange,
            BoardColor::Red,
            BoardColor::Yellow,
            BoardColor::Green,
            BoardColor::Blue,
        ]
    }

    pub fn get_property_colors() -> Vec<BoardColor> {
        vec![
            BoardColor::Railroad,
            BoardColor::Utilities,
            BoardColor::Brown,
            BoardColor::LightBlue,
            BoardColor::LightPurple,
            BoardColor::Orange,
            BoardColor::Red,
            BoardColor::Yellow,
            BoardColor::Green,
            BoardColor::Blue,
        ]
    }
}

pub trait BoardPlace {
    fn info(&self) -> String;
    fn get_id(&self) -> usize;
    fn get_place_name(&self) -> &'static str;
    fn get_action<'a>(&self, turn: usize, board: &Board) -> EventKind<'a>;
    fn get_owner(&self) -> Option<usize>;
    fn set_owner(&mut self, owner: Option<usize>);
    fn get_num_houses(&self) -> Option<u8>;
    fn set_num_houses(&mut self, num: u8);
    fn get_price_of_house(&self) -> Option<u32>;
    fn get_price(&self) -> u32;
    fn get_color(&self) -> BoardColor;
    fn is_mortgaged(&self) -> bool;
    fn set_mortgaged(&mut self, mortgaged: bool) -> u32;
}

impl dyn BoardPlace + Send {
    pub fn get_return_cost(&self) -> u32 {
        ((self.get_price() / 2) as f32 * 1.1) as u32
    }

    pub fn is_property(&self) -> bool {
        self.get_color() != BoardColor::None
    }

    pub fn is_estate(&self) -> bool {
        let color = self.get_color();
        color != BoardColor::None && color != BoardColor::Railroad && color != BoardColor::Utilities
    }

    pub fn get_rent(&self, board: &Board) -> Option<u32> {
        match self.get_action(usize::MAX, board) {
            EventKind::PayToBank(_, money) => Some(money),
            EventKind::PayToOther(_, _, money) => Some(money),
            _ => None,
        }
    }
}

pub fn get_place_list() -> Vec<Box<dyn BoardPlace + Send>> {
    vec![
        Nothing::new_boxed(0, "Go"),
        Estate::new_boxed(
            1,
            BoardColor::Brown,
            "Mediterranean Avenue",
            60,
            50,
            vec![2, 4, 10, 30, 90, 160, 250],
        ),
        CommunityChest::new_boxed(2),
        Estate::new_boxed(
            3,
            BoardColor::Brown,
            "Baltic Avenue",
            60,
            50,
            vec![4, 8, 20, 60, 180, 320, 450],
        ),
        IncomeTax::new_boxed(4),
        Railroad::new_boxed(5, "Reading Railroad"),
        Estate::new_boxed(
            6,
            BoardColor::LightBlue,
            "Oriental Avenue",
            100,
            50,
            vec![6, 12, 30, 90, 270, 400, 550],
        ),
        Chance::new_boxed(7),
        Estate::new_boxed(
            8,
            BoardColor::LightBlue,
            "Vermont Avenue",
            100,
            50,
            vec![6, 12, 30, 90, 270, 400, 550],
        ),
        Estate::new_boxed(
            9,
            BoardColor::LightBlue,
            "Connecticut Avenue",
            120,
            50,
            vec![8, 16, 40, 100, 300, 450, 600],
        ),
        Nothing::new_boxed(10, "Just Visiting"),
        Estate::new_boxed(
            11,
            BoardColor::LightPurple,
            "St. Charles Place",
            140,
            100,
            vec![10, 20, 50, 150, 450, 625, 750],
        ),
        Utilities::new_boxed(12, "Electric Company"),
        Estate::new_boxed(
            13,
            BoardColor::LightPurple,
            "States Avenue",
            140,
            100,
            vec![10, 20, 50, 150, 450, 625, 750],
        ),
        Estate::new_boxed(
            14,
            BoardColor::LightPurple,
            "Virginia Avenue",
            160,
            100,
            vec![12, 24, 60, 180, 500, 700, 900],
        ),
        Railroad::new_boxed(15, "Pennsylvania Railroad"),
        Estate::new_boxed(
            16,
            BoardColor::Orange,
            "St. James Place",
            180,
            100,
            vec![14, 28, 70, 200, 550, 750, 950],
        ),
        CommunityChest::new_boxed(17),
        Estate::new_boxed(
            18,
            BoardColor::Orange,
            "Tennessee Avenue",
            180,
            100,
            vec![14, 28, 70, 200, 550, 750, 950],
        ),
        Estate::new_boxed(
            19,
            BoardColor::Orange,
            "New York Avenue",
            200,
            100,
            vec![16, 32, 80, 220, 600, 800, 1000],
        ),
        Nothing::new_boxed(20, "Free Parking"),
        Estate::new_boxed(
            21,
            BoardColor::Red,
            "Kentucky Avenue",
            220,
            150,
            vec![18, 36, 90, 250, 700, 875, 1050],
        ),
        Chance::new_boxed(22),
        Estate::new_boxed(
            23,
            BoardColor::Red,
            "Indiana Avenue",
            220,
            150,
            vec![18, 36, 90, 250, 700, 875, 1050],
        ),
        Estate::new_boxed(
            24,
            BoardColor::Red,
            "Illinois Avenue",
            240,
            150,
            vec![20, 40, 100, 300, 750, 925, 1100],
        ),
        Railroad::new_boxed(25, "B. & O. Railroad"),
        Estate::new_boxed(
            26,
            BoardColor::Yellow,
            "Atlantic Avenue",
            260,
            150,
            vec![22, 44, 110, 330, 800, 975, 1150],
        ),
        Estate::new_boxed(
            27,
            BoardColor::Yellow,
            "Ventnor Avenue",
            260,
            150,
            vec![22, 44, 110, 330, 800, 975, 1150],
        ),
        Utilities::new_boxed(28, "Water Works"),
        Estate::new_boxed(
            29,
            BoardColor::Yellow,
            "Marvin Gardens",
            280,
            150,
            vec![24, 48, 120, 360, 850, 1025, 1200],
        ),
        GoToJail::new_boxed(30),
        Estate::new_boxed(
            31,
            BoardColor::Green,
            "Pacific Avenue",
            300,
            200,
            vec![26, 52, 130, 390, 900, 1100, 1275],
        ),
        Estate::new_boxed(
            32,
            BoardColor::Green,
            "North Carolina Avenue",
            300,
            200,
            vec![26, 52, 130, 390, 900, 1100, 1275],
        ),
        CommunityChest::new_boxed(33),
        Estate::new_boxed(
            34,
            BoardColor::Green,
            "Pennsylvania Avenue",
            320,
            200,
            vec![28, 56, 150, 450, 1000, 1200, 1400],
        ),
        Railroad::new_boxed(35, "Short Line"),
        Chance::new_boxed(36),
        Estate::new_boxed(
            37,
            BoardColor::Blue,
            "Park Place",
            350,
            200,
            vec![35, 70, 175, 500, 1100, 1300, 1500],
        ),
        LuxuryTax::new_boxed(38),
        Estate::new_boxed(
            39,
            BoardColor::Blue,
            "Boardwalk",
            400,
            200,
            vec![50, 100, 200, 600, 1400, 1700, 2000],
        ),
    ]
}
