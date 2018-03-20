#![feature(plugin)]
#![plugin(rocket_codegen)]

// --- External Libraries ---
// --------------------------

extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate serde_json;
extern crate rocket_contrib;
use rocket_contrib::Json;
use serde_json::Value;

// --- Structures ---
// ------------------

/**
 * Stuct with deserialize from this json
 * {
 *  "game_id": 1,
 *  "width": 20,
 *  "height": 20
 * }
 */
#[derive(Deserialize)]
struct StartReq {
    game_id: i32,
    width: i32,
    height: i32,
}

/**
 * Struct with serialize into json with this format
 * {
 *   "color": "#FF0000",
 *   "secondary_color": "#00FF00",
 *   "head_url": "http://www.rustacean.net/more-crabby-things/animated-ferris.gif",
 *   "taunt": "OH GOD NOT THE BEES",
 *   "head_type": "pixel",
 *   "tail_type": "pixel"
 * }
 */
#[derive(Serialize)]
struct StartResp {
    color: String,
    secondary_color: String,
    head_url: String,
    taunt: String,
    head_type: String,
    tail_type: String,
}

#[derive(Serialize, Deserialize)]
struct MoveResp {
    movement: String,
    taunt: String,
}

// --- HTTP HANDLERS ---
// ---------------------

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/**
 * This starts the game.
 * The battlesnake server will give us:
 * {
 *   "game_id": 1,
 *   "width": 20,
 *   "height": 20
 * }
 * 
 * The server expects us to response with a json in this format:
 * {
 *   "color": "#FF0000",
 *   "secondary_color": "#00FF00",
 *   "head_url": "http://placecage.com/c/100/100",
 *   "taunt": "OH GOD NOT THE BEES",
 *   "head_type": "pixel",
 *   "tail_type": "pixel"
 * }
 */
#[post("/start", data="<StartReq>")]
fn start(StartReq: Json<StartReq>) -> Json<Value> {
    Json(json!(StartResp {
        color: String::from("#d87b1e"),
        secondary_color: String::from("#13a341"),
        head_url: String::from("http://www.rustacean.net/more-crabby-things/animated-ferris.gif"),
        taunt: String::from("Rust: fast, reliable, productive—pick three."),
        head_type: String::from("fang"),
        tail_type: String::from("pixel"),
    }))
}

#[post("/move")]
fn movement() -> Json<Value> {
    Json(json!(MoveResp {
        movement: String::from("up"),
        taunt: String::from("Hello"),
    }))
}

/*
#[post("/end")]
fn end() {

}
*/

fn main() {
    rocket::ignite().mount("/", routes![index, start, movement]).launch();
}