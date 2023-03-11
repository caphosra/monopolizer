use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use places::BoardColor;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Rect};
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{Frame, Terminal};

use crate::board::Board;
use crate::player::Player;

pub mod actions;
pub mod appraiser;
pub mod board;
pub mod dice_rolling;
pub mod places;
pub mod player;
pub mod strategy;

fn main() -> Result<(), Box<dyn Error>> {
    let mut board: Option<Board> = None;
    loop {
        if let Some(board) = &mut board {
            for log in board.get_logs() {
                println!("{}", log);
            }
        }

        print!("> ");
        stdout().flush().unwrap();

        let mut line = String::new();
        let stdin = stdin();
        stdin.lock().read_line(&mut line).unwrap();
        line.pop();

        let args: Vec<&str> = line.split(" ").collect();

        if args.len() == 1 && args[0] == "exit" {
            break;
        }
        if args.len() == 2 && args[0] == "init" {
            if let Ok(place) = args[1].parse::<usize>() {
                let players: Vec<Player> = (0..place).map(|id| Player::new(id)).collect();
                board = Some(Board::new(players));
            }
        }
        if args.len() == 1 && args[0] == "step" {
            if let Some(board) = &mut board {
                board.spend_one_turn()
            }
        }
        if args.len() == 1 && args[0] == "v" {
            if let Some(board) = &mut board {
                render(board)?;
            }
        }
    }
    Ok(())
}

fn tui_logs<'a, B: Backend>(
    board: &'a Board,
    current_scroll: &'a mut i32,
) -> impl FnOnce(&mut Frame<B>) + 'a {
    move |f: &mut Frame<B>| {
        let paragraph = Paragraph::new(
            board
                .get_logs()
                .iter()
                .skip(*current_scroll as usize)
                .take(f.size().height as usize)
                .map(|f| f.clone())
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .alignment(Alignment::Left);

        f.render_widget(paragraph, f.size());
    }
}

fn tui_board<'a, B: Backend>(board: &'a mut Board) -> impl FnOnce(&mut Frame<B>) + 'a {
    move |f: &mut Frame<B>| {
        let layouts: Vec<Vec<Rect>> = Layout::default()
            //.margin(10)
            .direction(tui::layout::Direction::Horizontal)
            .constraints(
                (0..11)
                    .map(|_| Constraint::Percentage(100 / 11))
                    .collect::<Vec<Constraint>>(),
            )
            .split(f.size())
            .iter()
            .map(|rect| {
                Layout::default()
                    //.margin(10)
                    .direction(tui::layout::Direction::Vertical)
                    .constraints(
                        (0..11)
                            .map(|_| Constraint::Percentage(100 / 11))
                            .collect::<Vec<Constraint>>(),
                    )
                    .split(*rect)
            })
            .collect();

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

                    let players_on_place = board.get_players_on_place(place.get_id());
                    let players_on_place = players_on_place
                        .iter()
                        .map(|player| player.player_id.to_string())
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
                    let paragraph =
                        Paragraph::new(vec![owner, current_status, players_on_place].join("\n"))
                            .block(
                                Block::default()
                                    .title(place_name)
                                    .title_alignment(Alignment::Center)
                                    .borders(Borders::ALL)
                                    .border_style(Style::default().fg(color)),
                            )
                            .alignment(Alignment::Center);

                    f.render_widget(paragraph, layouts[x][y]);
                }
            }
        }
    }
}

fn render(board: &mut Board) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut terminal_selection = 0;
    let mut current_scroll = 0;

    terminal.clear()?;
    loop {
        match terminal_selection {
            0 => terminal.draw(tui_board(board)),
            1 => terminal.draw(tui_logs(board, &mut current_scroll)),
            _ => panic!("The tui needed is invalid."),
        }?;

        if let Event::Key(key) = read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            if key.code == KeyCode::Char('s') {
                board.spend_one_turn();
            }
            if key.code == KeyCode::Up {
                current_scroll -= 1;
                if current_scroll < 0 {
                    current_scroll = 0;
                }
            }
            if key.code == KeyCode::Down {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    current_scroll = board.get_logs().len() as i32;
                } else {
                    current_scroll += 1;
                }
            }
            if key.code == KeyCode::Left {
                terminal_selection = 0;
            }
            if key.code == KeyCode::Right {
                terminal_selection = 1;
            }
        }
    }

    disable_raw_mode()?;

    terminal.clear()?;

    Ok(())
}
