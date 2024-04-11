use rocket::serde::json::Json;
use rocket::http::Status;
use reqwest::{Client};
use rocket::serde::json::Value;
use rocket::serde::json::json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

// 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

const PORT_NUMBER: u16 = 5000;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState{
    pub connect_4: bool, // TRUE FOR CONNECT 4
    pub board_state: Vec<Vec<i32>>, // BOARD STATE
    pub difficulty: u32, // 1 FOR EASY; 2 FOR HARD
    pub error: i32 // -1 indicates error // CHECK THIS FIELD BEFORE USING API RESPONSE
}

async fn get_connect_4_updated_game_state(game_state: GameState) -> Result<Value, reqwest::Error>{
    let url = format!("http://localhost:{PORT_NUMBER}/project/connect");
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

    

#[tokio::main]
async fn main(){

    // TEST BOARD STATE
    let x1 = vec![1, 0, 0, 0, 2];
    let x2 = vec![2, 1, 2, 1, 1];
    let x3 = vec![2, 2, 1, 2, 1];
    let x4 = vec![2, 1, 2, 2, 1];

    let game_state: GameState = GameState{
        connect_4: true, // TRUE FOR CONNECT 4
        board_state: vec![x1, x2, x3, x4], // BOARD STATE
        difficulty: 2, // 1 FOR EASY; 2 FOR HARD
        error: 0
    };

    let next_game_state = match get_connect_4_updated_game_state(game_state).await {
        Ok(value) => {
            match serde_json::from_value::<GameState>(value) {
                Ok(parsed) => parsed,
                Err(err) => {
                    println!("Error parsing JSON: {:?}", err);
                    return;
                }
            }
        },
        Err(err) => {
            // Handle the error if needed
            println!("Error: {:?}", err);
            return;
        }
    };

    for row in &next_game_state.board_state {
        println!("{:?}", row);
    }
}