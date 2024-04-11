// async fn generate_text(game_state: String){
//     let api_key = "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";
//     let api_url = "https://api.openai.com/v1/chat/completions";

//     let prompt = "You are Player 2. Your objective is to play (and maybe win) the game of Connect 4. 
//                   you must place your token '2' on the game board i.e., replce a 0 with '2'
//                   You can only place 2 where there is an existing 0.
//                   You cannot place a 2 where there was not a prexisting zero.
//                   You HAVE to replace an existing zero with your token '2'.
//                   Make your move now and provide only the updated game state.";

//     println!("{prompt}");
//     let client = reqwest::Client::new();
//     let response = client
//         .post(api_url)
//         .header("Authorization", format!("Bearer {}", api_key))
//         .header("Content-Type", "application/json")
//         .json(&json!({
//             "model": "gpt-4-turbo", // Using a different model for testing
//             "max_tokens": 500,
//             "messages": [
//                 {"role": "system", "content": prompt},
//                 {"role": "user", "content": format!("{}", game_state)}
//             ],
//             "temperature": 0.2, // Example temperature value
//             "stop": ["\n"] // Ensure the response ends at a newline
//         }))
//         .send()
//         .await
//         .unwrap();


//     let text = response.text().await.unwrap();
//     println!("{:?}", text);
// }





// // async fn get_connect_4_game_state() -> Result<Value, reqwest::Error>{

// //     let x1 = vec![0, 0, 0, 0];
// //     let x2 = vec![0, 0, 0, 0];
// //     let x3 = vec![0, 0, 0, 0];
// //     let x4 = vec![1, 2, 2, 0];

// //     let param: GameState = GameState{
// //         connect_4: true, // TRUE FOR CONNECT 4
// //         board_state: vec![x1, x2, x3, x4], // BOARD STATE
// //         difficulty: 2, // 1 FOR EASY; 2 FOR HARD
// //     };

// //     let url = format!("http://localhost:{PORT_NUMBER}/project/connect");
// //     let client = reqwest::Client::new();
// //     let res = client.post(url)
// //         .json(&param)
// //         .send()
// //         .await
// //         .unwrap();
// //         // .unwrap().text().await;

// //     match res.status() {
// //         reqwest::StatusCode::OK => {
// //             match res.json::<GameState>().await {
// //                 Ok(parsed) => println!("Success! {:?}", parsed),
// //                 Err(_) => println!("Unexpected Error"),
// //             };
// //         }
// //         other => {
// //             panic!("Error happened! {:?}", other);
// //         }
// //     };

// //     Ok(().into())
// // }
