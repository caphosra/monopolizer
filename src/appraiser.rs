use crate::board::Board;
use crate::events::EventKind;
use crate::places::BoardColor;
use crate::player::Player;

///
/// Contains utilities that can be used for appraising properties owned by a player.
///
pub struct Appraiser;

impl Appraiser {
    ///
    /// Calculates the maximum price which the player can pay with mortgaging their properties.
    ///
    /// Note that this method regards mortgaged places as no more than $0, even if they are valuable for some points.
    ///
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

    ///
    /// Calculates the index named TAP(Total Attacking Point), sum of the rents of their properties.
    ///
    /// It is a simple yet helpful index that is considered to be approximating the income of the player well.
    ///
    pub fn get_tap(player: &Player, board: &Board) -> u32 {
        let mut tap = 0;
        let mut num_of_utilities = 0;

        // Sum up the rent for each property.
        for place in &board.places {
            if place.get_owner() == Some(player.player_id) {
                // Rents of utilities will be calculated by a different way.
                if place.get_color() != BoardColor::Utilities {
                    // Get the rent of the property by having an imaginary player land on it.
                    //
                    // To act as an imaginary player, give `usize::Max`, which cannot be `player_id`, as `player_id`
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

        // Calculates the expectation of the rent of utility.
        tap += match num_of_utilities {
            0 => 0,
            1 => 28,
            2 => 70 * 2,
            _ => panic!("The number of utilities must be less than or equal to 2."),
        };

        tap
    }
}
