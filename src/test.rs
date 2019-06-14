use super::rocket;
use crate::responses;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn ping() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let response = client.post("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn start() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let mut response = client
        .post("/start")
        .header(ContentType::JSON)
        .body(
            r#"{
            "game": {
                "id": "game-id-string"
            },
            "turn": 4,
            "board": {
                "height": 15,
                "width": 15,
                "food": [
                {
                    "x": 1,
                    "y": 3
                }
                ],
                "snakes": [
                    {
                        "id": "snake-id-string",
                        "name": "Sneky Snek",
                        "health": 90,
                        "body": [
                            {
                                "x": 1,
                                "y": 3
                            }
                        ]
                    }
                ]
            },
            "you": {
                "id": "snake-id-string",
                "name": "Sneky Snek",
                "health": 90,
                "body": [
                {
                    "x": 1,
                    "y": 3
                }
                ]
            }
        }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // test the response to match the regex
    let _start: responses::Start = serde_json::from_str(&response.body_string().unwrap()).unwrap();
}

#[test]
fn movement() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let mut response = client
        .post("/move")
        .header(ContentType::JSON)
        .body(
            r#"{
            "game": {
                "id": "game-id-string"
            },
            "turn": 4,
            "board": {
                "height": 15,
                "width": 15,
                "food": [
                {
                    "x": 1,
                    "y": 3
                }
                ],
                "snakes": [
                    {
                        "id": "snake-id-string",
                        "name": "Sneky Snek",
                        "health": 90,
                        "body": [
                            {
                                "x": 1,
                                "y": 3
                            }
                        ]
                    }
                ]
            },
            "you": {
                "id": "snake-id-string",
                "name": "Sneky Snek",
                "health": 90,
                "body": [
                {
                    "x": 1,
                    "y": 3
                }
                ]
            }
        }"#,
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // test the response to match the regex
    let body = response.body_string().unwrap();
    let _move: responses::Move = serde_json::from_str(&body).unwrap();
}

#[test]
fn end() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let response = client.post("/end").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
