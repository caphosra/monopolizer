use crate::board::Board;
use crate::events::EventKind;
use crate::places::BoardColor;
use crate::player::Player;

pub struct Appraiser;

impl Appraiser {
    pub fn get_payable_money(player: &Player, board: &Board) -> u32 {
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

    pub fn get_tap(player: &Player, board: &Board) -> u32 {
        let mut tap = 0;
        let mut num_of_utilities = 0;
        for place in &board.places {
            if place.get_owner() == Some(player.player_id) {
                if place.get_color() != BoardColor::Utilities {
                    match place.get_action(usize::MAX, board) {
                        EventKind::PayToOther(_, player_id, money) => {
                            assert_eq!(player_id, player.player_id);

                            tap += money;
                        }
                        _ => {}
                    };
                } else {
                    num_of_utilities += 1;
                }
            }
        }

        tap += match num_of_utilities{
            0 => 0,
            1 => 28,
            2 => 70 * 2,
            _ => panic!("The number of utilities must be less than or equal to 2.")
        };

        tap
    }
}
