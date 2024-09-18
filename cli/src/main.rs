pub mod renderer;

use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

use crate::renderer::start_render_loop;
use mplz_core::board::GameSession;
use mplz_core::command::{AnalysisCommandArg, GameCommand};

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
            (["init" | "i", player_num], game) => {
                if let Ok(player_num) = player_num.parse::<u32>() {
                    GameCommand::Init(player_num, game).execute()?;
                }
            }
            (["step" | "s", step], Some(game)) => {
                if let Ok(step) = step.parse::<u32>() {
                    GameCommand::Step(step, game).execute()?;
                }
            }
            (["vmode" | "v"], Some(game)) => {
                start_render_loop(game)?;
            }
            (["save" | "w", file_name], Some(game)) => {
                GameCommand::Save(file_name, game).execute()?;
            }
            (["load" | "r", file_name], game) => {
                GameCommand::Load(file_name, game).execute()?;
            }
            (["analyze" | "a", file_name, iteration, simulation_turn], Some(game)) => {
                let file_name = file_name.to_string();
                let iteration: i32 = iteration.parse().unwrap();
                let simulation_turn: usize = simulation_turn.parse().unwrap();

                let arg = AnalysisCommandArg {
                    file_name,
                    iteration,
                    simulation_turn,
                };

                GameCommand::Analyze(arg, game).execute()?;
            }
            _ => println!("Unknown command."),
        }
    }
    Ok(())
}
