use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::board::GameSession;

///
/// Holds arguments of analysis command.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisCommandArg {
    pub file_name: String,
    pub iteration: i32,
    pub simulation_turn: usize,
}

impl std::fmt::Display for AnalysisCommandArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

///
/// A command which the user can call through the prompt.
///
pub enum GameCommand<'a> {
    Init(u32, &'a mut Option<GameSession>),
    Step(u32, &'a mut GameSession),
    ModifyMoney {
        player_id: usize,
        money: i32,
        session: &'a mut GameSession,
    },
    SetOwner {
        player_id: usize,
        place_id: usize,
        session: &'a mut GameSession,
    },
    Save(&'a str, &'a GameSession),
    Load(&'a str, &'a mut Option<GameSession>),
    Analyze(AnalysisCommandArg, &'a GameSession),
}

impl<'a> GameCommand<'a> {
    ///
    /// Executes a command.
    ///
    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Init(player_num, session) => {
                **session = Some(GameSession::new(*player_num));
            }
            Self::Step(step, session) => {
                for _ in 0..*step {
                    session.spend_one_turn();
                }
            }
            Self::ModifyMoney {
                player_id,
                money,
                session,
            } => {
                let player = session.get_player_mut(*player_id);
                let modified = player.money as i32 + *money;
                if modified < 0 {
                    session
                        .logs
                        .push(format!("[PLAYER{}] Failed to pay ${}.", player_id, -*money))
                } else {
                    player.money = modified as u32;
                }
            }
            Self::SetOwner {
                player_id,
                place_id,
                session,
            } => {
                session.board.places[*place_id].set_owner(Some(*player_id));

                let place_name = session.board.places[*place_id].get_place_name();
                session.logs.push(format!(
                    "[PLAYER{}] Become an owner of {}.",
                    player_id, place_name
                ))
            }
            Self::Save(file_name, session) => {
                let json = session.to_json();
                let mut f = File::create(file_name)?;
                f.write_all(json.as_bytes())?;
            }
            Self::Load(file_name, session) => {
                let extension = Path::new(&file_name).extension().unwrap().to_str().unwrap();
                match extension {
                    "json" => {
                        let mut f = File::open(file_name)?;
                        let mut json = String::new();
                        f.read_to_string(&mut json)?;

                        **session = Some(GameSession::from_json(&json));
                    }
                    "xlsx" => {
                        **session = Some(GameSession::from_excel(file_name));
                    }
                    ext => {
                        println!("Files with \".{}\" are not supported.", ext)
                    }
                }
            }
            Self::Analyze(arg, session) => {
                let mut result = String::new();
                result += "turn,player,money,tap\n";

                let json = session.to_json();
                for _ in 0..arg.iteration {
                    let mut game = GameSession::from_json(&json);
                    for i in 0..arg.simulation_turn {
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

                let mut f = File::create(&arg.file_name)?;
                f.write_all(result.as_bytes())?;
            }
        }

        Ok(())
    }
}
