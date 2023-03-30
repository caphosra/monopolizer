pub mod appraiser;
pub mod board;
pub mod dice_rolling;
pub mod events;
pub mod places;
pub mod player;
pub mod renderer;
pub mod serialization;
pub mod strategy;

use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, Read, Write};

use crate::board::GameSession;
use crate::renderer::start_render_loop;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game: Option<GameSession> = None;
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut line = String::new();
        let stdin = stdin();
        stdin.lock().read_line(&mut line).unwrap();
        line.pop();

        let args: Vec<&str> = line.split(" ").collect();

        match (&args[..], &mut game) {
            (["exit"] | ["q"], _) => break,
            (["init" | "i", player_num], _) => {
                if let Ok(player_num) = player_num.parse::<u32>() {
                    game = Some(GameSession::new(player_num));
                }
            }
            (["step" | "s", step], Some(game)) => {
                if let Ok(step) = step.parse::<i32>() {
                    for _ in 0..step {
                        game.spend_one_turn();
                    }
                }
            }
            (["vmode" | "v"], Some(game)) => {
                start_render_loop(game)?;
            }
            (["save" | "w", file_name], Some(game)) => {
                let json = game.to_json();
                let mut f = File::create(file_name)?;
                f.write_all(json.as_bytes())?;
            }
            (["load" | "r", file_name], _) => {
                let mut f = File::open(file_name)?;
                let mut json = String::new();
                f.read_to_string(&mut json)?;

                game = Some(GameSession::from_json(&json));
            }
            (["analyze" | "a", file_name, iteration, turn_num], Some(game)) => {
                let iterations: i32 = iteration.parse().unwrap();
                let turn_num: usize = turn_num.parse().unwrap();

                let mut result = String::new();
                result += "turn,player,money,tap\n";

                let json = game.to_json();
                for _ in 0..iterations {
                    let mut game = GameSession::from_json(&json);
                    for i in 0..turn_num {
                        game.spend_one_turn();

                        let summaries = game.export_summaries(i);
                        let summaries = summaries
                            .iter()
                            .map(|summary| summary.to_string())
                            .collect::<Vec<_>>()
                            .join("\n");

                        result += &summaries;
                        result += "\n";
                    }
                }

                let mut f = File::create(file_name)?;
                f.write_all(result.as_bytes())?;
            }
            _ => println!("Unknown command."),
        }
    }
    Ok(())
}
