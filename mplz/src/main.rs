use std::sync::Mutex;

use actix_web::web::Json;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::Lazy;

use mplzlib::board::GameSession;
use mplzlib::command::{AnalysisCommandArg, GameCommand};

const MONOPOLY_PORT: u16 = 5391;

static GLOBAL_GAME_SESSION: Lazy<Mutex<Option<GameSession>>> = Lazy::new(|| Mutex::new(None));

#[post("/init")]
async fn init(player_num: String) -> impl Responder {
    match player_num.parse::<u32>() {
        Ok(player_num) => {
            let mut board = GLOBAL_GAME_SESSION.lock().unwrap();

            match GameCommand::Init(player_num, &mut board).execute() {
                Ok(_) => {
                    let msg = format!("Successfully initialized with {} player(s).", player_num);
                    println!("[/init] {}", msg);
                    HttpResponse::Ok().body(msg)
                }
                Err(_) => HttpResponse::BadRequest().body("Failed to initialize the board."),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("The number of player is not valid."),
    }
}

#[post("/step")]
async fn step(step: String) -> impl Responder {
    match step.parse::<u32>() {
        Ok(step) => {
            let mut board = GLOBAL_GAME_SESSION.lock().unwrap();

            match board.as_mut() {
                Some(board) => match GameCommand::Step(step, board).execute() {
                    Ok(_) => {
                        let msg = format!("Stepped {} times.", step);
                        println!("[/step] {}", msg);
                        HttpResponse::Ok().body(msg)
                    }
                    Err(_) => HttpResponse::BadRequest().body("Failed to initialize the board."),
                },
                None => HttpResponse::BadRequest().body("The board has not been initialized."),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("The number of player is not valid."),
    }
}

#[post("/save")]
async fn save(file_name: String) -> impl Responder {
    let mut board = GLOBAL_GAME_SESSION.lock().unwrap();

    match board.as_mut() {
        Some(board) => match GameCommand::Save(&file_name, board).execute() {
            Ok(_) => {
                let msg = format!("Successfully saved to \"{}\".", file_name);
                println!("[/save] {}", msg);
                HttpResponse::Ok().body(msg)
            }
            Err(_) => HttpResponse::BadRequest().body("Failed to save the board to the file."),
        },
        None => HttpResponse::BadRequest().body("The board has not been initialized."),
    }
}

#[post("/load")]
async fn load(file_name: String) -> impl Responder {
    let mut board = GLOBAL_GAME_SESSION.lock().unwrap();

    match GameCommand::Load(&file_name, &mut board).execute() {
        Ok(_) => {
            let msg = format!("Successfully loaded \"{}\".", file_name);
            println!("[/load] {}", msg);
            HttpResponse::Ok().body(msg)
        }
        Err(_) => HttpResponse::BadRequest().body("Failed to load the board from the file."),
    }
}

#[post("/analyze")]
async fn analyze(arg: Json<AnalysisCommandArg>) -> impl Responder {
    let board = GLOBAL_GAME_SESSION.lock().unwrap();

    match board.as_ref() {
        Some(board) => match GameCommand::Analyze(arg.0, board).execute() {
            Ok(_) => {
                let msg = format!("Successfully analyzed.");
                println!("[/analyze] {}", msg);
                HttpResponse::Ok().body(msg)
            }
            Err(_) => HttpResponse::BadRequest().body("Failed to load the board from the file."),
        },
        None => HttpResponse::BadRequest().body("The board has not been initialized."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arg = AnalysisCommandArg {
        file_name: "some".to_owned(),
        iteration: 10,
        simulation_turn: 10,
    };

    println!("{}", arg.to_string());
    println!("Starting the server...");

    HttpServer::new(|| {
        App::new()
            .service(init)
            .service(step)
            .service(save)
            .service(load)
            .service(analyze)
    })
    .bind(("127.0.0.1", MONOPOLY_PORT))?
    .run()
    .await
}
