use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Start {
    color: String,
    #[serde(rename = "headType")]
    head_type: HeadType,
    #[serde(rename = "tailType")]
    tail_type: TailType,
}

impl Start {
    pub fn new(color: String, head_type: HeadType, tail_type: TailType) -> Start {
        Start {
            color,
            head_type,
            tail_type,
        }
    }
}

// TODO: Make all the head types
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HeadType {
    Regular,
    Beluga,
    Bendr,
    Dead,
    Evil,
    Fang,
    Pixel,
    Safe,
    #[serde(rename = "sand-worm")]
    SandWorm,
    Shades,
    Smile,
    Tongue,
}

// TODO: Make all the tail types
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TailType {
    Regular,
    #[serde(rename = "block-bum")]
    BlockBum,
    Bolt,
    Curled,
    #[serde(rename = "fat-rattle")]
    FatRattle,
    Freckled,
    Hook,
    Pixel,
    #[serde(rename = "round-bum")]
    RoundBum,
    Sharp,
    Skinny,
    #[serde(rename = "small-rattle")]
    SmallRattle,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Move {
    #[serde(rename = "move")]
    movement: Movement,
}

impl Move {
    pub fn new(movement: Movement) -> Move {
        Move { movement }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_start() {
        let response = Start {
            color: "#ff00ff".to_string(),
            head_type: HeadType::Bendr,
            tail_type: TailType::Pixel,
        };

        let correct_serialized_response =
            "{\"color\":\"#ff00ff\",\"headType\":\"bendr\",\"tailType\":\"pixel\"}";

        println!("{}", correct_serialized_response);

        match serde_json::to_string(&response) {
            Err(e) => {
                eprintln!("Returned value is Err: {}", e);
                assert!(false);
            }
            Ok(val) => {
                assert_eq!(correct_serialized_response, val);
            }
        }
    }

    #[test]
    fn serialize_move() {
        let response = Move {
            movement: Movement::Right,
        };

        let correct_serialized_response = "{\"move\":\"right\"}";

        match serde_json::to_string(&response) {
            Err(e) => {
                eprintln!("Returned value is Err: {}", e);
                assert!(false);
            }
            Ok(val) => {
                assert_eq!(correct_serialized_response, val);
            }
        }
    }

    #[test]
    fn deserialize_start() {
        let string = "{\"color\":\"#ff00ff\",\"headType\":\"bendr\",\"tailType\":\"pixel\"}";

        let deserialized_start = serde_json::from_str(&string).unwrap();
        let correct_start = Start::new(String::from("#ff00ff"), HeadType::Bendr, TailType::Pixel);
        assert_eq!(correct_start, deserialized_start);
    }

    #[test]
    fn deserialize_move() {
        let string = "{\"move\":\"right\"}";

        let deserialized_move = serde_json::from_str(&string).unwrap();
        let correct_move = Move {
            movement: Movement::Right,
        };
        assert_eq!(correct_move, deserialized_move);
    }

}
