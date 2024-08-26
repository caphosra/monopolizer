use actix_files::Files;
use actix_web::web::{Json, Query, Redirect};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use mplzlib::board::GameSession;
use mplzlib::command::AnalysisCommandArg;
use mplzlib::serialization::{GameInfo, PlaceProp};

const MONOPOLY_PORT: u16 = 5391;

#[get("/")]
async fn root() -> impl Responder {
    Redirect::to("/index.html")
}

#[derive(Deserialize)]
struct InitQuery {
    num: u32,
}

#[get("/init")]
async fn init(query: Query<InitQuery>) -> impl Responder {
    let board = GameSession::new(query.num);
    board.to_json()
}

#[derive(Deserialize)]
struct StepBody {
    game: GameInfo,
    num: u32,
}

#[post("/step")]
async fn step(body: Json<StepBody>) -> impl Responder {
    let mut session = GameSession::from_info(&body.game);
    for _ in 0..body.num {
        session.spend_one_turn();
    }
    HttpResponse::Ok().body(session.to_json())
}

#[derive(Serialize)]
struct PlacesBody {
    places: Vec<PlaceProp>,
}

#[post("/places")]
async fn places(body: Json<GameInfo>) -> impl Responder {
    let session = GameSession::from_info(&body);
    let places = session
        .board
        .places
        .iter()
        .map(|place| place.to_place_prop(&session.board))
        .collect::<Vec<_>>();
    let body = PlacesBody { places };
    HttpResponse::Ok().body(serde_json::to_string_pretty(&body).unwrap())
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
            .service(root)
            .service(init)
            .service(step)
            .service(places)
            .service(
                Files::new("/", "./web/build/")
                    .prefer_utf8(true)
                    .show_files_listing(),
            )
    })
    .bind(("127.0.0.1", MONOPOLY_PORT))?
    .run()
    .await
}
