#[macro_use] extern crate rocket;

use rocket::http::Method;
use rocket::{post, routes};
use rocket::serde::json::Json;
use crate::model::{ColResponse, GameResult};
use crate::utils::puff_connect4;
use rocket_cors::{AllowedOrigins, AllowedHeaders, CorsOptions, Cors};

mod model;
mod utils;

#[post("/puff_step", format = "json", data = "<game_board_state>")]
fn connect_puff_move(game_board_state: Json<GameResult>) -> Json<ColResponse> {
    let GameResult { game_board, difficulty_level } = game_board_state.into_inner();
    println!("Received game board state: {:?}", game_board);
    let column = puff_connect4(game_board, difficulty_level);
    Json(ColResponse { column })
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
        "http://127.0.0.1:8080",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("Error while building CORS")
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![connect_puff_move]).attach(make_cors())
}

