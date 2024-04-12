/*
 * File: chat_gpt.rs
 * Author: Fahrin Bushra
 * Date: April 11, 2024
 * Adapted From: CHAT GPT
 * Description: TOOT OTTO RESPONSE FROM GPT-4 TURBO
*/

use serde_json::json;
use rocket::serde::json::Value;

pub async fn generate_text(game_state: String) -> Vec<Vec<i32>>{
    let api_key = "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";
    let api_url = "https://api.openai.com/v1/chat/completions";

    let prompt = "Your objective is to play (and maybe win) the game of TOOT OTTO
                  To do that, you must achieve the sequence -1 1 1 -1.
                  To do that, you must achieve the sequence -1 1 1 -1.
                  Place a 1 or a -1 on one of the locations where zero currently exists
                  DO NOT MODIFY EXISTING 1 or -1
                  DO NOT MODIFY EXISTING 1 or -1
                  You HAVE to replace an existing zero with a '1' or '-1'.
                  You must place your token '-1' or '1' on the game board i.e., replace a 0 with a '1' or '-1'
                  Make your move now and provide only the updated game state with your response added.";
                  ;

    // println!("{prompt}");
    let client = reqwest::Client::new();
    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "gpt-4-turbo", // Using a different model for testing
            "max_tokens": 500,
            "messages": [
                {"role": "system", "content": prompt},
                {"role": "user", "content": format!("{}", game_state)}
            ],
            "temperature": 0.3, // Example temperature value
            "stop": ["\n"] // Ensure the response ends at a newline
        }))
        .send()
        .await
        .unwrap();
    
    // EXTRACT THE CONTENT FIELD FROM THE JSON RESPONSE
    let parsed_json: Value = serde_json::from_str(response.text().await.unwrap().as_str()).unwrap();
    let content = parsed_json["choices"][0]["message"]["content"].as_str().unwrap();

    // CONVERT THE 2D VECTOR JSON STRING INTO A 2D VECTOR
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let elements: Vec<&str> = content.trim_start_matches('[').trim_end_matches(']').split("],[").collect();
    for row_str in elements {
        let row: Vec<i32> = row_str.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        matrix.push(row);
    }

    println!("GPT MATRIX {:?}", matrix);
    return matrix.clone();
}





// async fn get_connect_4_game_state() -> Result<Value, reqwest::Error>{
//     let x1 = vec![0, 0, 0, 0];
//     let x2 = vec![0, 0, 0, 0];
//     let x3 = vec![0, 0, 0, 0];
//     let x4 = vec![1, 2, 2, 0];

//     let param: GameState = GameState{
//         connect_4: true, // TRUE FOR CONNECT 4
//         board_state: vec![x1, x2, x3, x4], // BOARD STATE
//         difficulty: 2, // 1 FOR EASY; 2 FOR HARD
//     };

//     let url = format!("http://localhost:{PORT_NUMBER}/project/connect");
//     let client = reqwest::Client::new();
//     let res = client.post(url)
//         .json(&param)
//         .send()
//         .await
//         .unwrap();
//         // .unwrap().text().await;

//     match res.status() {
//         reqwest::StatusCode::OK => {
//             match res.json::<GameState>().await {
//                 Ok(parsed) => println!("Success! {:?}", parsed),
//                 Err(_) => println!("Unexpected Error"),
//             };
//         }
//         other => {
//             panic!("Error happened! {:?}", other);
//         }
//     };

//     Ok(().into())
// }
