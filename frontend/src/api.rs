use serde::{Deserialize, Serialize};
use serde_json::Value;

const PORT_NUMBER: u16 = 5000;
const EMPTY: i32 = 0;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub connect_4: bool,            // TRUE FOR CONNECT 4
    pub board_state: Vec<Vec<i32>>, // BOARD STATE
    pub difficulty: u32,            // 1 FOR EASY; 2 FOR HARD
    pub error: i32, // -1 indicates error // CHECK THIS FIELD BEFORE USING API RESPONSE
}

pub async fn get_connect4_computer_move(game_state: GameState) -> Result<Value, reqwest::Error> {
    let url = format!("http://localhost:{PORT_NUMBER}/project/connect4");
    let client = reqwest::Client::new();
    let res = match client.get(url).json(&game_state).send().await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    match res.status() {
        reqwest::StatusCode::OK => {
            match res.json::<GameState>().await {
                Ok(parsed) => {
                    println!("Success!");
                    return Ok(serde_json::to_value(parsed).unwrap());
                }
                Err(_) => println!("Unexpected Error"),
            };
        }
        other => {
            panic!("Error happened! {:?}", other);
        }
    };

    Ok(Value::Null)
}
