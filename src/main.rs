#![feature(plugin)]
#![plugin(rocket_codegen)]

// --- External Libraries ---
// --------------------------

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use rocket_contrib::Json;
use serde_json::Value;
use serde::ser::{Serialize, Serializer, SerializeStruct};

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

/**
 * Struct with Deserialization from this Json:
 * {
 *  "food": {
 *    "data": [
 *      {
 *        "object": "point",
 *        "x": 0,
 *        "y": 9
 *      }
 *    ],
 *    "object": "list"
 *  },
 *  "height": 20,
 *  "id": 1,
 *  "object": "world",
 *  "snakes": {
 *    "data": [
 *      {
 *        "body": {
 *          "data": [
 *            {
 *              "object": "point",
 *              "x": 13,
 *              "y": 19
 *            },
 *            {
 *              "object": "point",
 *              "x": 13,
 *              "y": 19
 *            },
 *            {
 *              "object": "point",
 *              "x": 13,
 *              "y": 19
 *            }
 *          ],
 *          "object": "list"
 *        },
 *        "health": 100,
 *        "id": "58a0142f-4cd7-4d35-9b17-815ec8ff8e70",
 *        "length": 3,
 *        "name": "Sonic Snake",
 *        "object": "snake",
 *        "taunt": "Gotta go fast"
 *      },
 *      {
 *        "body": {
 *          "data": [
 *            {
 *              "object": "point",
 *              "x": 8,
 *              "y": 15
 *            },
 *            {
 *              "object": "point",
 *              "x": 8,
 *              "y": 15
 *            },
 *            {
 *              "object": "point",
 *              "x": 8,
 *              "y": 15
 *            }
 *          ],
 *          "object": "list"
 *        },
 *        "health": 100,
 *        "id": "48ca23a2-dde8-4d0f-b03a-61cc9780427e",
 *        "length": 3,
 *        "name": "Typescript Snake",
 *        "object": "snake",
 *        "taunt": ""
 *      }
 *    ],
 *    "object": "list"
 *  },
 *  "turn": 0,
 *  "width": 20,
 *  "you": {
 *    "body": {
 *      "data": [
 *        {
 *          "object": "point",
 *          "x": 8,
 *          "y": 15
 *        },
 *        {
 *          "object": "point",
 *          "x": 8,
 *          "y": 15
 *        },
 *        {
 *          "object": "point",
 *          "x": 8,
 *          "y": 15
 *        }
 *      ],
 *      "object": "list"
 *    },
 *    "health": 100,
 *    "id": "48ca23a2-dde8-4d0f-b03a-61cc9780427e",
 *    "length": 3,
 *    "name": "Typescript Snake",
 *    "object": "snake",
 *    "taunt": ""
 *  }
 * }
 */
#[derive(Deserialize)]
struct MoveReq {
    food: Points,
    height: i64,
    id: i64,
    object: String,
    snakes: Snakes,
    turn: i64,
    width: i64,
    you: Snake,
}

/**
 * This struct stores a list of points
 * Points can be food or a body part
 */
#[derive(Deserialize)]
struct Points {
    data: Vec<Point>,
    object: String,
}

#[derive(Deserialize)]
struct Snakes {
    data: Vec<Snake>,
    object: String,
}

/**
 * This struct is just a point on the board
 */
#[derive(Deserialize)]
struct Point {
    object: String,
    x: i64,
    y: i64,
}

#[derive(Deserialize)]
struct Snake {
  body: Points,
  health: i64,
  id: String,
  length: i64,
  name: String,
  object: String,
  taunt: String,
}

/**
 * This is the struct that turns into the json to send back to the server
 * move is a keyword in rust so movement is used but the serializer will 
 * convert it into the correct format
 */
struct MoveResp {
    movement: String,
    taunt: String,
}

/**
 * This is the method to serialize the struct
 */
impl Serialize for MoveResp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("MoveResp", 2)?;
        state.serialize_field("move", &self.movement)?;
        state.serialize_field("taunt", &self.taunt)?;
        state.end()
    }
}


/**
 * Deserializes json into struct
 * {
 *   "game_id": 10,
 *   "winners": [ "a46b558b-f31b-418f-bb07-6017dd91f653" ],
 *   "dead_snakes": {
 *     "object": "list",
 *     "data": [{
 *       "id": "4a35fd1c-434b-431b-839c-edf958d67e9a",
 *       "length": 3,
 *       "death": {
 *         "turn": 4,
 *         "causes": ["self collision"]
 *       }
 *     }]
 *   }
 * } 
 */
#[derive(Deserialize)]
struct EndReq {
    game_id: i64,
    winners: Vec<String>,
    dead_snakes: DeadSnakes,
}

/**
 * This is just a list of dead snakes
 */
#[derive(Deserialize)]
struct DeadSnakes {
    object: String,
    data: Vec<DeadSnake>,
}

/**
 * Dead snakes aren't the same struct as normal snakes.
 */
#[derive(Deserialize)]
struct DeadSnake {
    id: String,
    length: i64,
    death: Death,
}

/**
 * Death consists of the turn number and a list of strings that are the causes
 * causes of death could be: body collison, head collison, self collison,
 * starvation, and wall collison.
 */
#[derive(Deserialize)]
struct Death {
  turn: i64,
  causes: Vec<String>,
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

#[post("/move", data="<MoveReq>")]
fn movement(MoveReq: Json<MoveReq>) -> Json<Value> {
    println!("-------------------------------------");
    println!("TESTS");
    println!("-------------------------------------");
    println!("The game ID is: {}", MoveReq.id);
    println!("The width is: {} and height is: {}", MoveReq.width, MoveReq.height);
    println!("It is turn: {}", MoveReq.turn);
    println!("-------------------------------------");
    println!("INFORMATION ABOUT YOUR SNAKE");
    println!("-------------------------------------");
    println!("Name: {}", MoveReq.you.name);
    println!("Length: {}", MoveReq.you.length);
    println!("Health: {}", MoveReq.you.health);
    println!("Taunt: {}", MoveReq.you.taunt);
    println!("-------------------------------------");
    println!("END TESTS");
    println!("-------------------------------------");
    Json(json!(MoveResp {
        movement: String::from("up"),
        taunt: String::from("Hello"),
    }))
}


#[post("/end", data="<EndReq>")]
fn end(EndReq: Json<EndReq>) {
}

fn main() {
    rocket::ignite().mount("/", routes![index, start, movement, end]).launch();
}