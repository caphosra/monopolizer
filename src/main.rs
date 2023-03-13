pub mod actions;
pub mod appraiser;
pub mod board;
pub mod dice_rolling;
pub mod places;
pub mod player;
pub mod renderer;
pub mod strategy;

use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

use crate::board::MonopolyGame;
use crate::renderer::start_render_loop;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game: Option<MonopolyGame> = None;
    loop {
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
            if let Ok(player_num) = args[1].parse::<u32>() {
                game = Some(MonopolyGame::new(player_num));
            }
        }
        if args.len() == 1 && args[0] == "step" {
            if let Some(game) = &mut game {
                game.spend_one_turn();
            }
        }
        if args.len() == 2 && args[0] == "step" {
            if let Some(game) = &mut game {
                if let Ok(count) = args[1].parse::<i32>() {
                    for _ in 0..count {
                        game.spend_one_turn();
                    }
                }
            }
        }
        if args.len() == 1 && args[0] == "v" {
            if let Some(game) = &mut game {
                start_render_loop(game)?;
            }
        }
    }
    Ok(())
}
