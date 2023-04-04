use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use mplzlib::appraiser::Appraiser;
use mplzlib::board::Board;
use mplzlib::places::{BoardColor, BoardPlace};
use mplzlib::player::{Player, PlayerState};

///
/// Renders a place.
///
pub fn render_place<'a, B: Backend>(
    f: &mut Frame<B>,
    place: &'a Box<dyn BoardPlace>,
    players: &'a Vec<Player>,
    area: Rect,
) {
    let owner = place
        .get_owner()
        .map_or("owner: X".to_string(), |owner| format!("owner: {}", owner));

    let mortgaged = place.is_mortgaged();
    let houses = place.get_num_houses();

    let current_status = match (mortgaged, houses) {
        (true, _) => "MORTGAGED".to_string(),
        (false, Some(houses)) => "#".repeat(houses as usize),
        (false, None) => "".to_string(),
    };

    let players_on_place = players
        .iter()
        .filter_map(|player| {
            if player.position == place.get_id() {
                Some(player.player_id.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(",");

    let color = match place.get_color() {
        BoardColor::Brown => Color::Gray,
        BoardColor::LightBlue => Color::Cyan,
        BoardColor::LightPurple => Color::LightMagenta,
        BoardColor::Orange => Color::LightRed,
        BoardColor::Red => Color::Red,
        BoardColor::Yellow => Color::Yellow,
        BoardColor::Green => Color::Green,
        BoardColor::Blue => Color::Blue,
        BoardColor::Railroad => Color::DarkGray,
        BoardColor::Utilities => Color::DarkGray,
        _ => Color::Reset,
    };

    let mut place_name = place.get_place_name().to_string();
    place_name.truncate(8);
    let paragraph = Paragraph::new(vec![owner, current_status, players_on_place].join("\n"))
        .block(
            Block::default()
                .title(place_name)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color)),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

///
/// Gets a renderer for the board.
///
pub fn get_board_renderer<'a, B: Backend>(
    turn: usize,
    players: &'a Vec<Player>,
    board: &'a mut Board,
) -> impl FnOnce(&mut Frame<B>) + 'a {
    move |f| {
        let layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(81),
                Constraint::Percentage(9),
            ])
            .split(f.size());

        let center_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(81),
                Constraint::Percentage(9),
            ])
            .split(layouts[1]);

        let above_side_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                (0..9)
                    .map(|_| Constraint::Percentage(11))
                    .collect::<Vec<Constraint>>(),
            )
            .split(center_layouts[0]);

        let bottom_side_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                (0..9)
                    .map(|_| Constraint::Percentage(11))
                    .collect::<Vec<Constraint>>(),
            )
            .split(center_layouts[2]);

        let center_layout = center_layouts[1];

        let player_infos_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .margin(5)
            .split(center_layout);

        let paragraph = Paragraph::new(format!("Player{}", turn))
            .block(
                Block::default()
                    .title("NEXT TURN")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightBlue)),
            )
            .alignment(Alignment::Center);
        f.render_widget(paragraph, player_infos_layouts[0]);

        let player_num = players.len();
        let player_infos_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                (0..player_num)
                    .map(|_| Constraint::Percentage(100 / player_num as u16))
                    .collect::<Vec<Constraint>>(),
            )
            .split(player_infos_layouts[1]);

        for (i, player) in players.iter().enumerate().into_iter() {
            let (player_state, color) = match player.state {
                PlayerState::Bankrupted => ("BANKRUPTED".to_string(), Color::Red),
                PlayerState::InJail(turn) => (
                    format!("IN JAIL (Turn remained: {})", 3 - turn),
                    Color::Yellow,
                ),
                PlayerState::None => ("ACTIVE".to_string(), Color::Green),
            };

            let paragraph = Paragraph::new(format!(
                "${} (${} TAP:{})\nSTATUS: {}",
                player.money,
                Appraiser::get_payable_money(player, board),
                Appraiser::get_tap(player, board),
                player_state
            ))
            .block(
                Block::default()
                    .title(format!("Player{}", player.player_id))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color)),
            )
            .alignment(Alignment::Center);

            f.render_widget(paragraph, player_infos_layouts[i])
        }

        let left_side_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                (0..11)
                    .map(|_| Constraint::Percentage(9))
                    .collect::<Vec<Constraint>>(),
            )
            .split(layouts[0]);

        let right_side_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                (0..11)
                    .map(|_| Constraint::Percentage(9))
                    .collect::<Vec<Constraint>>(),
            )
            .split(layouts[2]);

        for x in 0..11 {
            for y in 0..11 {
                let place = match (x, y) {
                    (x, 0) => Some(&board.places[20 + x]),
                    (x, 10) => Some(&board.places[10 - x]),
                    (0, y) => Some(&board.places[20 - y]),
                    (10, y) => Some(&board.places[30 + y]),
                    _ => None,
                };

                if let Some(place) = place {
                    match (x, y) {
                        (0, y) => render_place(f, place, players, left_side_layouts[y]),
                        (10, y) => render_place(f, place, players, right_side_layouts[y]),
                        (x, 0) => render_place(f, place, players, above_side_layouts[x - 1]),
                        (x, 10) => render_place(f, place, players, bottom_side_layouts[x - 1]),
                        _ => {}
                    };
                }
            }
        }
    }
}
