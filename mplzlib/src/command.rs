use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use crate::board::GameSession;

///
/// Holds arguments of analysis command.
///
pub struct AnalysisCommandArg<'a> {
    pub file_name: &'a str,
    pub iteration: i32,
    pub simulation_turn: usize,
}

///
/// A command which the user can call through the prompt.
///
pub enum GameCommand<'a> {
    Init(u32, &'a mut Option<GameSession>),
    Step(u32, &'a mut GameSession),
    Save(&'a str, &'a GameSession),
    Load(&'a str, &'a mut Option<GameSession>),
    Analyze(AnalysisCommandArg<'a>, &'a GameSession),
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
            Self::Save(file_name, session) => {
                let json = session.to_json();
                let mut f = File::create(file_name)?;
                f.write_all(json.as_bytes())?;
            }
            Self::Load(file_name, session) => {
                let mut f = File::open(file_name)?;
                let mut json = String::new();
                f.read_to_string(&mut json)?;

                **session = Some(GameSession::from_json(&json));
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

                let mut f = File::create(arg.file_name)?;
                f.write_all(result.as_bytes())?;
            }
        }

        Ok(())
    }
}
