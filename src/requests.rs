use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Turn {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Snake,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Game {
    pub id: String,
    pub timeout: i32,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Point>,
    pub snakes: Vec<Snake>,
    pub hazards: Vec<Point>,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Snake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Point>,
    pub head: Point,
    pub length: u32,
    pub shout: String,
    pub squad: String,
    pub latency: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_turn() {
        let turn1 = r#"{
            "game": {
              "id": "game-00fe20da-94ad-11ea-bb37",
              "ruleset": {
                "name": "standard",
                "version": "v.1.2.3"
              },
              "timeout": 500
            },
            "turn": 14,
            "board": {
              "height": 11,
              "width": 11,
              "food": [
                {"x": 5, "y": 5}, 
                {"x": 9, "y": 0}, 
                {"x": 2, "y": 6}
              ],
              "hazards": [
                {"x": 3, "y": 2}
              ],
              "snakes": [
                {
                  "id": "snake-508e96ac-94ad-11ea-bb37",
                  "name": "My Snake",
                  "health": 54,
                  "body": [
                    {"x": 0, "y": 0}, 
                    {"x": 1, "y": 0}, 
                    {"x": 2, "y": 0}
                  ],
                  "latency": "111",
                  "head": {"x": 0, "y": 0},
                  "length": 3,
                  "shout": "why are we shouting??",
                  "squad": ""
                }, 
                {
                  "id": "snake-b67f4906-94ae-11ea-bb37",
                  "name": "Another Snake",
                  "health": 16,
                  "body": [
                    {"x": 5, "y": 4}, 
                    {"x": 5, "y": 3}, 
                    {"x": 6, "y": 3},
                    {"x": 6, "y": 2}
                  ],
                  "latency": "222",
                  "head": {"x": 5, "y": 4},
                  "length": 4,
                  "shout": "I'm not really sure...",
                  "squad": ""
                }
              ]
            },
            "you": {
              "id": "snake-508e96ac-94ad-11ea-bb37",
              "name": "My Snake",
              "health": 54,
              "body": [
                {"x": 0, "y": 0}, 
                {"x": 1, "y": 0}, 
                {"x": 2, "y": 0}
              ],
              "latency": "111",
              "head": {"x": 0, "y": 0},
              "length": 3,
              "shout": "why are we shouting??",
              "squad": ""
            }
          }"#;

        let game = Game {
            id: "game-00fe20da-94ad-11ea-bb37".to_string(),
            timeout: 500,
        };

        let you = Snake {
            id: "snake-508e96ac-94ad-11ea-bb37".to_string(),
            name: "My Snake".to_string(),
            health: 54,
            body: vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
            ],
            latency: "111".to_string(),
            head: Point { x: 0, y: 0 },
            length: 3,
            shout: "why are we shouting??".to_string(),
            squad: "".to_string(),
        };

        let snake = Snake {
            id: "snake-b67f4906-94ae-11ea-bb37".to_string(),
            name: "Another Snake".to_string(),
            health: 16,
            body: vec![
                Point { x: 5, y: 4 },
                Point { x: 5, y: 3 },
                Point { x: 6, y: 3 },
                Point { x: 6, y: 2 },
            ],
            length: 4,
            head: Point { x: 5, y: 4 },
            latency: "222".to_string(),
            shout: "I'm not really sure...".to_string(),
            squad: "".to_string(),
        };

        let board = Board {
            height: 11,
            width: 11,
            food: vec![
                Point { x: 5, y: 5 },
                Point { x: 9, y: 0 },
                Point { x: 2, y: 6 },
            ],
            hazards: vec![Point { x: 3, y: 2 }],
            snakes: vec![you.clone(), snake],
        };

        let correct: Turn = Turn {
            game: game,
            turn: 14,
            board: board,
            you: you,
        };

        let result: serde_json::Result<Turn> = serde_json::from_str(turn1);
        match result {
            Err(e) => {
                eprintln!("Returned value is Err: {}", e);
                assert!(false);
            }
            Ok(val) => {
                // println!("{:#?}", correct);
                println!("{:#?}", val);
                assert_eq!(correct, val);
            }
        }
    }
}
