use serde::{Deserialize};

#[derive(Deserialize)]
struct Point(i32, i32);

#[derive(Deserialize)]
struct Turn {
    game_id: String,
    turn: u32,
    board: Board,
    you: Snake,
}

#[derive(Deserialize)]
struct Board {
    height: i32,
    width: i32,
    food: Vec<Point>,
    snakes: Vec<Snake>,
}

#[derive(Deserialize)]
struct Snake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Point>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_deserialize()  {
        let turn1 = r#"{
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
        }"#;

        let result: serde_json::Result<Turn> = serde_json::from_str(turn1);
        match result {
            Err(e) => {
                eprintln!("{}", e);
                assert!(false);
            },
            Ok(val) => {
                return;
            }
        }
    }
}