use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Turn {
    game: Game,
    turn: u32,
    board: Board,
    you: Snake,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Game {
    id: String,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Board {
    height: i32,
    width: i32,
    food: Vec<Point>,
    snakes: Vec<Snake>,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Snake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Point>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_turn() {
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

        let correct: Turn = Turn {
            game: Game {
                id: "game-id-string".to_string(),
            },
            turn: 4,
            board: Board {
                height: 15,
                width: 15,
                food: vec![Point { x: 1, y: 3 }],
                snakes: vec![Snake {
                    id: "snake-id-string".to_string(),
                    name: "Sneky Snek".to_string(),
                    health: 90,
                    body: vec![Point { x: 1, y: 3 }],
                }],
            },
            you: Snake {
                id: "snake-id-string".to_string(),
                name: "Sneky Snek".to_string(),
                health: 90,
                body: vec![Point { x: 1, y: 3 }],
            },
        };

        let result: serde_json::Result<Turn> = serde_json::from_str(turn1);
        match result {
            Err(e) => {
                eprintln!("Returned value is Err: {}", e);
                assert!(false);
            }
            Ok(val) => {
                assert_eq!(correct, val);
            }
        }
    }
}
