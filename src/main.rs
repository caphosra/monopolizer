use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

use crate::board::Board;
use crate::player::Player;
use crate::renderer::start_render_loop;

pub mod actions;
pub mod appraiser;
pub mod board;
pub mod dice_rolling;
pub mod places;
pub mod player;
pub mod renderer;
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
        if args.len() == 2 && args[0] == "step" {
            if let Some(board) = &mut board {
                if let Ok(count) = args[1].parse::<i32>() {
                    for _ in 0..count {
                        board.spend_one_turn()
                    }
                }
            }
        }
        if args.len() == 1 && args[0] == "v" {
            if let Some(board) = &mut board {
                start_render_loop(board)?;
            }
        }
    }
    Ok(())
}
