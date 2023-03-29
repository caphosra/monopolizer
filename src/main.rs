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

        match args[..] {
            ["exit"] | ["q"] => break,
            ["init" | "i", num] => {
                if let Ok(player_num) = num.parse::<u32>() {
                    game = Some(MonopolyGame::new(player_num));
                }
            }
            ["step" | "s"] => {
                if let Some(game) = &mut game {
                    if let Ok(count) = args[1].parse::<i32>() {
                        for _ in 0..count {
                            game.spend_one_turn();
                        }
                    }
                }
            }
            ["vmode" | "v"] => {
                if let Some(game) = &mut game {
                    start_render_loop(game)?;
                }
            }
            ["save" | "w", file_name] => {
                if let Some(game) = &mut game {
                    let json = game.to_json();
                    let mut f = File::create(file_name)?;
                    f.write_all(json.as_bytes())?;
                }
            }
            ["load" | "r", file_name] => {
                let mut f = File::open(file_name)?;
                let mut json = String::new();
                f.read_to_string(&mut json)?;

                game = Some(MonopolyGame::from_json(&json));
            }
            ["analyze" | "a", file_name, iteration, turn_num] => {
                if let Some(game) = &mut game {
                    let iterations: i32 = iteration.parse().unwrap();
                    let turn_num: i32 = turn_num.parse().unwrap();

                    let mut result = String::new();
                    result += "turn,player,money,tap\n";

                    let json = game.to_json();
                    for _ in 0..iterations {
                        let mut game = MonopolyGame::from_json(&json);
                        for i in 0..turn_num {
                            game.spend_one_turn();

                            for player in &game.players {
                                let money_infos =
                                    Appraiser::get_payable_money(player, &game.board).to_string();
                                let tap = Appraiser::get_tap(player, &game.board);
                                result += &format!(
                                    "{},{},{},{}\n",
                                    i + 1,
                                    player.player_id,
                                    money_infos,
                                    tap
                                );
                            }
                        }
                    }

                    let mut f = File::create(file_name)?;
                    f.write_all(result.as_bytes())?;
                }
            }
            _ => println!("Unknown command."),
        }
    }
    Ok(())
}
