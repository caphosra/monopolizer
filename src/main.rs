pub mod actions;
pub mod appraiser;
pub mod board;
pub mod dice_rolling;
pub mod places;
pub mod player;
pub mod renderer;
pub mod serialization;
pub mod strategy;

use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, Read, Write};

use crate::appraiser::Appraiser;
use crate::board::MonopolyGame;
use crate::renderer::start_render_loop;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game: Option<MonopolyGame> = None;
    loop {
        print!("$ ");
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
        if args.len() == 2 && args[0] == "save" {
            if let Some(game) = &mut game {
                let json = game.to_json();
                let mut f = File::create(args[1])?;
                f.write_all(json.as_bytes())?;
            }
        }
        if args.len() == 2 && args[0] == "load" {
            let mut f = File::open(args[1])?;
            let mut json = String::new();
            f.read_to_string(&mut json)?;

            game = Some(MonopolyGame::from_json(&json));
        }
        if args.len() == 4 && args[0] == "analyze" {
            if let Some(game) = &mut game {
                let iterations: i32 = args[2].parse().unwrap();
                let turn_num: i32 = args[3].parse().unwrap();

                let mut result = String::new();
                result += "turn,player,money\n";

                let json = game.to_json();
                for _ in 0..iterations {
                    let mut game = MonopolyGame::from_json(&json);
                    for i in 0..turn_num {
                        game.spend_one_turn();

                        for player in &game.players {
                            let money_infos = Appraiser::appraise(player, &game.board).to_string();
                            result += &format!("{},{},{}\n", i + 1, player.player_id, money_infos);
                        }
                    }
                }

                let mut f = File::create(args[1])?;
                f.write_all(result.as_bytes())?;
            }
        }
    }
    Ok(())
}
