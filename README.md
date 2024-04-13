### **MAJOR INNOVATIONS**
- Our Toot Otto game variant's API's hard mode computer moves are GPT assisted
- We utilized ChatGPT to get the code for how to communicate with GPT-4-Turbo API

### **RATIONALE FOR DESIGN DECISIONS**
What can we do on a computer that we can’t do on a printed board?
1
ECE 421 | Exploring Software Development Domains
2) What is a computerized opponent? What are its objectives? (Remember, not everyone is an expert.)
- What characteristics should it possess? Do we need an opponent or opponents?
3) What design choices exist for the Interface components?
- Color? Font? Dimensions of Windows? Rescale-ability? Scroll Bars? ….
4) What does exception handling mean in a GUI system?
5) Do we require a command-line interface for debugging purposes????? (The answer is yes by the
way – please explain why)


### **SYSTEM LIMITATIONS**
- The Toot-Otto computer minimax algorithm has a random component to it
    - From our research, this also seemed to be present in the given MEAN stack repo
    - The computer chooses a T or O token at random during its moves
    - Consequently, the algorithm sometimes spits out random moves instead of trying to win

- Our Toot OTTO game variant does not let the user select T or O tokens at each turn.
    - The user is fixed with Token T
    - We did not have enough time to implement user token selection

- API issues with reqwest
    - The reqwest library seemed to have dependency issues with yew
    - hence, we could not integrate our frontend with the API
    - So we copied backend functions onto our frontend and went with a simple MVC app
    - Unfortunately, this also meant that our TooT oTTo game's hard mode werenot GPT assisted in our frontend version (since GPT requires api calls that cannot be done without reqwest)


### **USER MANUAL**
1. Clone the repository
2. Run BACKEND SERVER:
    1. Ensure you are in the connect_api directory: `cd connect_api`
    2. Run the server: `cargo run`

3. Run FRONTEND:
    1. Open a separate terminal window
    2. Ensure you are in the frontend directory: `cd ../frontend`
    3. Make sure Trunk is installed with `cargo install trunk`
    4. Add wasm build target with `rustup target add wasm32-unknown-unknown`
    5. Run application using `trunk serve --open`

### **MARKETING VIDEO**
