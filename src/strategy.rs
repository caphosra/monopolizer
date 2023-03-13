use crate::board::Board;
use crate::places::BoardColor;
use crate::player::PlayerState;

pub trait ArrangementStrategy {
    fn raise(
        &self,
        debt: u32,
        board: &mut Board,
        player_id: usize,
        money: &mut u32,
        state: &PlayerState,
        position: usize,
    ) -> Result<(), u32>;
    //fn build(&self, board: &mut Board, money: &mut u32, state: &PlayerState, position: usize);
}

pub struct ExpensiveHousesProtectionStrategy;

impl ExpensiveHousesProtectionStrategy {
    pub fn new() -> Box<dyn ArrangementStrategy> {
        Box::new(ExpensiveHousesProtectionStrategy {})
    }
}

impl ArrangementStrategy for ExpensiveHousesProtectionStrategy {
    fn raise(
        &self,
        debt: u32,
        board: &mut Board,
        player_id: usize,
        money: &mut u32,
        _: &PlayerState,
        _: usize,
    ) -> Result<(), u32> {
        if *money >= debt {
            *money -= debt;
            return Ok(());
        }

        // Mortgages the places not monopolized.
        let players_places = board
            .places
            .iter()
            .filter(|place| {
                place.get_owner() == Some(player_id)
                    && !board.is_monopolized(place.get_color())
                    && !place.is_mortgaged()
            })
            .map(|place| place.get_id())
            .collect::<Vec<_>>();

        for place_id in players_places {
            let place = &mut board.places[place_id];
            *money += place.set_mortgaged(true);
            if *money >= debt {
                *money -= debt;
                return Ok(());
            }
        }

        let players_places = board
            .places
            .iter()
            .filter(|place| {
                place.get_owner() == Some(player_id)
                    && board.is_monopolized(place.get_color())
                    && !place.is_mortgaged()
                    && place.get_num_houses() == Some(0)
            })
            .map(|place| place.get_id())
            .collect::<Vec<_>>();

        for place_id in players_places {
            let place = &mut board.places[place_id];
            *money += place.set_mortgaged(true);
            if *money >= debt {
                *money -= debt;
                return Ok(());
            }
        }

        for color in BoardColor::get_estate_colors() {
            let mut places_id_houses = board
                .places
                .iter()
                .filter(|place| {
                    place.get_color() == color
                        && board.is_monopolized(place.get_color())
                        && !place.is_mortgaged()
                        && place.get_owner() == Some(player_id)
                })
                .map(|place| (place.get_id(), place.get_num_houses().unwrap()))
                .collect::<Vec<_>>();

            let mut houses: u8 = places_id_houses.iter().map(|(_, houses)| houses).sum();

            while houses > 0 {
                places_id_houses.sort_by(|(_, houses1), (_, houses2)| houses2.cmp(houses1));

                assert_ne!(places_id_houses.len(), 0);

                let (id, num_houses) = places_id_houses.first().unwrap();

                *money += board.places[*id].get_price_of_house().unwrap() / 2;
                board.places[*id].set_num_houses(num_houses - 1);

                if *money >= debt {
                    *money -= debt;
                    return Ok(());
                }

                houses -= 1;
            }

            // Mortgages the places. Prioritizes the cheaper place.
            places_id_houses.sort_by(|(id1, _), (id2, _)| id1.cmp(id2));
            for (id, _) in places_id_houses {
                *money += board.places[id].set_mortgaged(true);

                if *money >= debt {
                    *money -= debt;
                    return Ok(());
                }
            }
        }

        Err(*money)
    }
}
