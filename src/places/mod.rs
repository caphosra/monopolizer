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

use crate::actions::BoardAction;
use crate::board::Board;
use crate::places::chance::Chance;
use crate::places::community_chest::CommunityChest;
use crate::places::estate::Estate;
use crate::places::go_to_jail::GoToJail;
use crate::places::income_tax::IncomeTax;
use crate::places::luxury_tax::LuxuryTax;
use crate::places::nothing::Nothing;
use crate::places::railroad::Railroad;
use crate::places::utilities::Utilities;

#[derive(Debug, PartialEq, Eq, Clone)]
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
}

pub trait BoardPlace {
    fn info(&self) -> String;
    fn get_id(&self) -> usize;
    fn get_place_name(&self) -> &'static str;
    fn get_action<'a>(&self, turn: usize, board: &Board) -> BoardAction<'a>;
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

pub fn get_place_list() -> Vec<Box<dyn BoardPlace>> {
    vec![
        Nothing::new(0, "Go"),
        Estate::new(
            1,
            BoardColor::Brown,
            "Mediterranean Avenue",
            60,
            50,
            vec![2, 4, 10, 30, 90, 160, 250],
        ),
        CommunityChest::new(2),
        Estate::new(
            3,
            BoardColor::Brown,
            "Baltic Avenue",
            60,
            50,
            vec![4, 8, 20, 60, 180, 320, 450],
        ),
        IncomeTax::new(4),
        Railroad::new(5, "Reading Railroad"),
        Estate::new(
            6,
            BoardColor::LightBlue,
            "Oriental Avenue",
            100,
            50,
            vec![6, 12, 30, 90, 270, 400, 550],
        ),
        Chance::new(7),
        Estate::new(
            8,
            BoardColor::LightBlue,
            "Vermont Avenue",
            100,
            50,
            vec![6, 12, 30, 90, 270, 400, 550],
        ),
        Estate::new(
            9,
            BoardColor::LightBlue,
            "Connecticut Avenue",
            120,
            50,
            vec![8, 16, 40, 100, 300, 450, 600],
        ),
        Nothing::new(10, "Just Visiting"),
        Estate::new(
            11,
            BoardColor::LightPurple,
            "St. Charles Place",
            140,
            100,
            vec![10, 20, 50, 150, 450, 625, 750],
        ),
        Utilities::new(12, "Electric Company"),
        Estate::new(
            13,
            BoardColor::LightPurple,
            "States Avenue",
            140,
            100,
            vec![10, 20, 50, 150, 450, 625, 750],
        ),
        Estate::new(
            14,
            BoardColor::LightPurple,
            "Virginia Avenue",
            160,
            100,
            vec![12, 24, 60, 180, 500, 700, 900],
        ),
        Railroad::new(15, "Pennsylvania Railroad"),
        Estate::new(
            16,
            BoardColor::Orange,
            "St. James Place",
            180,
            100,
            vec![14, 28, 70, 200, 550, 750, 950],
        ),
        CommunityChest::new(17),
        Estate::new(
            18,
            BoardColor::Orange,
            "Tennessee Avenue",
            180,
            100,
            vec![14, 28, 70, 200, 550, 750, 950],
        ),
        Estate::new(
            19,
            BoardColor::Orange,
            "New York Avenue",
            200,
            100,
            vec![16, 32, 80, 220, 600, 800, 1000],
        ),
        Nothing::new(20, "Free Parking"),
        Estate::new(
            21,
            BoardColor::Red,
            "Kentucky Avenue",
            220,
            150,
            vec![18, 36, 90, 250, 700, 875, 1050],
        ),
        Chance::new(22),
        Estate::new(
            23,
            BoardColor::Red,
            "Indiana Avenue",
            220,
            150,
            vec![18, 36, 90, 250, 700, 875, 1050],
        ),
        Estate::new(
            24,
            BoardColor::Red,
            "Illinois Avenue",
            240,
            150,
            vec![20, 40, 100, 300, 750, 925, 1100],
        ),
        Railroad::new(25, "B. & O. Railroad"),
        Estate::new(
            26,
            BoardColor::Yellow,
            "Atlantic Avenue",
            260,
            150,
            vec![22, 44, 110, 330, 800, 975, 1150],
        ),
        Estate::new(
            27,
            BoardColor::Yellow,
            "Ventnor Avenue",
            260,
            150,
            vec![22, 44, 110, 330, 800, 975, 1150],
        ),
        Utilities::new(28, "Water Works"),
        Estate::new(
            29,
            BoardColor::Yellow,
            "Marvin Gardens",
            280,
            150,
            vec![24, 48, 120, 360, 850, 1025, 1200],
        ),
        GoToJail::new(30),
        Estate::new(
            31,
            BoardColor::Green,
            "Pacific Avenue",
            300,
            200,
            vec![26, 52, 130, 390, 900, 1100, 1275],
        ),
        Estate::new(
            32,
            BoardColor::Green,
            "North Carolina Avenue",
            300,
            200,
            vec![26, 52, 130, 390, 900, 1100, 1275],
        ),
        CommunityChest::new(33),
        Estate::new(
            34,
            BoardColor::Green,
            "Pennsylvania Avenue",
            320,
            200,
            vec![28, 56, 150, 450, 1000, 1200, 1400],
        ),
        Railroad::new(35, "Short Line"),
        Chance::new(36),
        Estate::new(
            37,
            BoardColor::Blue,
            "Park Place",
            350,
            200,
            vec![35, 70, 175, 500, 1100, 1300, 1500],
        ),
        LuxuryTax::new(38),
        Estate::new(
            39,
            BoardColor::Blue,
            "Boardwalk",
            400,
            200,
            vec![50, 100, 200, 600, 1400, 1700, 2000],
        ),
    ]
}
