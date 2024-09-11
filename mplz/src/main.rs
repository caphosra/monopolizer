use actix_cors::Cors;
use actix_files::Files;
use actix_web::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use actix_web::web::{Json, Query, Redirect};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use mplzlib::appraiser::Appraiser;
use mplzlib::player::PlayerState;
use serde::{Deserialize, Serialize};

use mplzlib::board::GameSession;
use mplzlib::serialization::{GameInfo, PlaceProp};

const MPLZ_API_PORT: u16 = 5391;

#[cfg(debug_assertions)]
const MPLZ_WEB_PORT: u16 = 5390;

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

#[derive(Serialize)]
struct TapBody {
    taps: Vec<u32>,
}

#[post("/tap")]
async fn tap(body: Json<GameInfo>) -> impl Responder {
    let session = GameSession::from_info(&body);
    let taps = session
        .players
        .iter()
        .map(|player| Appraiser::get_tap(player, &session.board))
        .collect();
    let body = TapBody { taps };
    HttpResponse::Ok().body(serde_json::to_string_pretty(&body).unwrap())
}

#[derive(Serialize)]
struct MoneyBody {
    money: Vec<u32>,
    available: Vec<u32>,
    total: Vec<u32>,
}

#[post("/money")]
async fn money(body: Json<GameInfo>) -> impl Responder {
    let session = GameSession::from_info(&body);
    let money = session
        .players
        .iter()
        .map(|player| player.money)
        .collect::<Vec<_>>();
    let total = session
        .players
        .iter()
        .map(|player| Appraiser::get_payable_money(player, &session.board))
        .collect::<Vec<_>>();
    let available = money
        .iter()
        .zip(total.iter())
        .map(|(money, total)| total - money)
        .collect();
    let body = MoneyBody {
        money,
        available,
        total,
    };
    HttpResponse::Ok().body(serde_json::to_string_pretty(&body).unwrap())
}

#[derive(Deserialize)]
struct SurvivalRequest {
    game: GameInfo,
    num: u32,
    depth: u32,
}

#[derive(Serialize)]
struct SurvivalResponse {
    survival_rates: Vec<f32>
}

#[post("/survival")]
async fn survival(body: Json<SurvivalRequest>) -> impl Responder {
    let mut counter = vec![0 as u32; body.game.players.len()];
    for _ in 0..body.num {
        let mut session = GameSession::from_info(&body.game);
        for _ in 0..body.depth {
            session.spend_one_turn();
        }
        for idx in 0..body.game.players.len() {
            let player = session.get_player(idx);
            match player.state {
                PlayerState::Bankrupted => { }
                _ => counter[idx] += 1,
            }
        }
    }
    let survival_rates = counter.iter().map(|&count| count as f32 / body.num as f32).collect();
    let body = SurvivalResponse { survival_rates };
    HttpResponse::Ok().body(serde_json::to_string_pretty(&body).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server...");

    HttpServer::new(|| {
        let cors = if cfg!(debug_assertions) {
            Cors::default()
                .allowed_origin(&format!("http://localhost:{}", MPLZ_WEB_PORT))
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allowed_headers(vec![AUTHORIZATION, ACCEPT])
                .allowed_header(CONTENT_TYPE)
                .max_age(3600)
        } else {
            Cors::default()
        };

        App::new()
            .wrap(cors)
            .service(root)
            .service(init)
            .service(step)
            .service(places)
            .service(tap)
            .service(money)
            .service(survival)
            .service(
                Files::new("/", "./web/build/")
                    .prefer_utf8(true)
                    .show_files_listing(),
            )
    })
    .bind(("127.0.0.1", MPLZ_API_PORT))?
    .run()
    .await
}
