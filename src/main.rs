use crate::board::Board;
use crate::player::Player;
use std::io::{self, stdout, BufRead, Write};

pub mod actions;
pub mod appraiser;
pub mod board;
pub mod dice_rolling;
pub mod places;
pub mod player;
pub mod strategy;

fn main() {
    let mut board: Option<Board> = None;
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut line = String::new();
        let stdin = io::stdin();
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

                println!("Initialized.");
            }
        }
        if args.len() == 1 && args[0] == "info" {
            if let Some(board) = &board {
                println!("{}", board.info());
            }
        }
        if args.len() == 1 && args[0] == "step" {
            if let Some(board) = &mut board {
                board.spend_one_turn()
            }
        }
    }
}
