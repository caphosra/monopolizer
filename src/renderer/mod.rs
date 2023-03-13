use std::error::Error;
use std::io::stdout;

use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::board::MonopolyGame;
use crate::renderer::board::get_board_renderer;
use crate::renderer::logs::get_logs_renderer;

mod board;
mod logs;

pub fn start_render_loop(game: &mut MonopolyGame) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut terminal_selection = 0;
    let mut current_scroll = 0;

    terminal.clear()?;
    loop {
        match terminal_selection {
            0 => terminal.draw(get_board_renderer(
                game.turn,
                &game.players,
                &mut game.board,
            )),
            1 => terminal.draw(get_logs_renderer(&game.logs, &mut current_scroll)),
            _ => panic!("The tui needed is invalid."),
        }?;

        if let Event::Key(key) = read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            if key.code == KeyCode::Char('s') {
                game.spend_one_turn();
            }
            if key.code == KeyCode::Up {
                current_scroll -= 1;
                if current_scroll < 0 {
                    current_scroll = 0;
                }
            }
            if key.code == KeyCode::Down {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    current_scroll = game.logs.len() as i32;
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
