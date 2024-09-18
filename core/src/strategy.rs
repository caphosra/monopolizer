use crate::board::Board;
use crate::places::BoardColor;
use crate::player::PlayerState;
use std::collections::HashSet;

///
/// Determines the player's behavior.
///
/// **Do not call functions with a suffix "_raw" directly, which are unintentionally exposed.**
///
pub trait PlayerStrategy {
    ///
    /// Raises money to pay off the debt.
    ///
    /// **Do not call directly. You should use `raise` instead.**
    ///
    fn raise_raw(
        &self,
        debt: u32,
        board: &mut Board,
        player_id: usize,
        money: &mut u32,
        state: &PlayerState,
        position: usize,
    ) -> Result<(), u32>;

    ///
    /// Does investment.
    ///
    /// **Do not call directly. You should use `invest` instead.**
    ///
    fn invest_raw(
        &self,
        board: &mut Board,
        player_id: usize,
        money: &mut u32,
        state: &PlayerState,
        position: usize,
    );
}

///
/// Quits the function after they pay off the debt. If they cannot do so, does nothing.
///
macro_rules! pay_off_and_quit {
    ($money: tt, $debt: tt) => {
        if *$money >= $debt {
            *$money -= $debt;
            return Ok(());
        }
    };
}

///
/// Quits the function if they don't have enough money to invest. If they have, do investment.
///
macro_rules! invest_or_quit {
    ($money:tt, $usable:tt, $cost:tt) => {
        if $cost <= $usable {
            $usable -= $cost;
            *$money -= $cost;
        } else {
            return;
        }
    };
}

impl dyn PlayerStrategy + Send {
    ///
    /// Raises money to pay off the debt.
    ///
    pub fn raise(
        &self,
        debt: u32,
        board: &mut Board,
        player_id: usize,
        money: &mut u32,
        state: &PlayerState,
        position: usize,
    ) -> Result<(), u32> {
        pay_off_and_quit!(money, debt);

        let result = self.raise_raw(debt, board, player_id, money, state, position);
        board.validate_houses();

        result
    }

    ///
    /// Does investment.
    ///
    pub fn invest(
        &self,
        board: &mut Board,
        player_id: usize,
        money: &mut u32,
        state: &PlayerState,
        position: usize,
    ) {
        self.invest_raw(board, player_id, money, state, position);
        board.validate_houses();
    }
}

pub struct ExpensiveHousesProtectionStrategy;

impl ExpensiveHousesProtectionStrategy {
    pub fn new_boxed() -> Box<dyn PlayerStrategy + Send> {
        Box::new(ExpensiveHousesProtectionStrategy {})
    }
}

impl PlayerStrategy for ExpensiveHousesProtectionStrategy {
    fn raise_raw(
        &self,
        debt: u32,
        board: &mut Board,
        player_id: usize,
        money: &mut u32,
        _: &PlayerState,
        _: usize,
    ) -> Result<(), u32> {
        let mut monopolized_color = HashSet::new();
        for color in BoardColor::get_estate_colors() {
            if board.get_monopolizer(color.clone()) == Some(player_id) {
                monopolized_color.insert(color);
            }
        }

        let mut players_places = board
            .places
            .iter_mut()
            .filter(|place| place.get_owner() == Some(player_id))
            .collect::<Vec<_>>();

        // Mortgages the places not monopolized.
        let players_not_monopolized_places = players_places.iter_mut().filter(|place| {
            !monopolized_color.contains(&place.get_color()) && !place.is_mortgaged()
        });

        for place in players_not_monopolized_places {
            assert!(place.get_num_houses().unwrap_or(0) == 0);

            *money += place.set_mortgaged(true);

            pay_off_and_quit!(money, debt);
        }

        let mut players_monopolized_places_with_houses = players_places
            .iter_mut()
            .filter_map(|place| {
                if monopolized_color.contains(&place.get_color()) && !place.is_mortgaged() {
                    Some((place.get_num_houses().unwrap(), place))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for color in BoardColor::get_estate_colors() {
            let mut color_places = players_monopolized_places_with_houses
                .iter_mut()
                .filter(|(_, place)| place.get_color() == color)
                .collect::<Vec<_>>();
            let sum_of_houses: u8 = color_places.iter().map(|(houses, _)| houses).sum();
            if sum_of_houses == 0 {
                for (_, place) in &mut color_places {
                    assert!(place.get_num_houses().unwrap_or(0) == 0);

                    *money += place.set_mortgaged(true);

                    pay_off_and_quit!(money, debt);
                }
            }
        }

        let mut players_monopolized_places_with_houses = players_monopolized_places_with_houses
            .iter_mut()
            .filter(|(_, place)| !place.is_mortgaged())
            .collect::<Vec<_>>();

        for color in BoardColor::get_estate_colors() {
            let mut color_places = players_monopolized_places_with_houses
                .iter_mut()
                .filter(|(_, place)| place.get_color() == color)
                .collect::<Vec<_>>();
            let mut sum_of_houses: u8 = color_places.iter().map(|(houses, _)| houses).sum();

            while sum_of_houses > 0 {
                color_places.sort_by(|(houses1, _), (houses2, _)| houses2.cmp(houses1));

                assert_ne!(color_places.len(), 0);

                let (houses, place) = color_places.first_mut().unwrap();

                assert!(*houses > 0u8);

                *money += place.get_price_of_house().unwrap() / 2;
                place.set_num_houses(*houses - 1);
                *houses -= 1;

                sum_of_houses -= 1;

                pay_off_and_quit!(money, debt);
            }

            // Mortgages the places. Prioritizes the cheaper place.
            color_places.sort_by(|(_, place1), (_, place2)| place1.get_id().cmp(&place2.get_id()));
            for (_, place) in color_places {
                assert!(place.get_num_houses().unwrap() == 0);
                *money += place.set_mortgaged(true);

                pay_off_and_quit!(money, debt);
            }
        }

        Err(*money)
    }

    fn invest_raw(
        &self,
        board: &mut Board,
        player_id: usize,
        money: &mut u32,
        _: &PlayerState,
        _: usize,
    ) {
        let usable = *money as i32 - board.get_most_expensive(player_id) as i32;
        if usable > 0 {
            let mut usable = usable as u32;

            let player_places = board
                .places
                .iter_mut()
                .filter(|place| place.get_owner() == Some(player_id));

            let mortgaged_railroads = player_places
                .filter(|place| place.get_color() == BoardColor::Railroad && place.is_mortgaged());
            for railroad in mortgaged_railroads {
                let cost = railroad.get_return_cost();

                invest_or_quit!(money, usable, cost);

                railroad.set_mortgaged(false);
            }

            let colors = BoardColor::get_estate_colors();
            let mut players_colors = colors
                .iter()
                .filter(|color| board.get_monopolizer((*color).clone()) == Some(player_id))
                .collect::<Vec<_>>();
            players_colors.sort_by_key(|color| {
                u8::MAX - board.get_houses_num_by_color((*color).clone()).unwrap()
            });

            for color in players_colors {
                let houses_limit =
                    board.gets_by_color(color.clone()).collect::<Vec<_>>().len() as u8 * 5;
                let mut houses = board.get_houses_num_by_color(color.clone()).unwrap();

                let mortgaged_places = board
                    .gets_by_color_mut(color.clone())
                    .filter(|place| place.is_mortgaged());
                for place in mortgaged_places {
                    let cost = place.get_return_cost();

                    invest_or_quit!(money, usable, cost);

                    place.set_mortgaged(false);
                }

                let mut places = board.gets_by_color_mut(color.clone()).collect::<Vec<_>>();
                while houses < houses_limit {
                    places.sort_by_key(|place| place.get_num_houses().unwrap());

                    let place_to_build = places.first_mut().unwrap();

                    assert!(!place_to_build.is_mortgaged());

                    let cost = place_to_build.get_price_of_house().unwrap();
                    invest_or_quit!(money, usable, cost);

                    place_to_build.set_num_houses(place_to_build.get_num_houses().unwrap() + 1);
                    houses += 1;

                    assert!(place_to_build.get_num_houses().unwrap() <= 5);
                }
            }

            let mortgaged_places = board
                .places
                .iter_mut()
                .filter(|place| place.get_owner() == Some(player_id) && place.is_mortgaged());

            for place in mortgaged_places {
                let cost = place.get_return_cost();
                invest_or_quit!(money, usable, cost);

                place.set_mortgaged(false);
            }
        }
    }
}
