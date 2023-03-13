use crate::{board::Board, player::Player};

pub struct Appraiser;

impl Appraiser {
    pub fn appraise(player: &Player, board: &Board) -> u32 {
        let mut price = player.money;

        price += board
            .places
            .iter()
            .filter(|place| place.get_owner() == Some(player.player_id) && !place.is_mortgaged())
            .map(|place| {
                place.get_price() / 2
                    + place.get_price_of_house().unwrap_or(0)
                        * place.get_num_houses().unwrap_or(0) as u32
                        / 2
            })
            .sum::<u32>();

        price
    }
}
