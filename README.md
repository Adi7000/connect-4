### **MAJOR INNOVATIONS**
- Our Toot Otto game variant's hard mode computer moves are GPT assisted
- We utilized ChatGPT to get the code for how to communicate with GPT-4-Turbo API

### **RATIONALE FOR DESIGN DECISIONS**
- Our system is divided into a frontend and a backend
- Frontend consists of UI components and basic validation functions regarding game state
    - The frontend makes API calls to the backend to get computer moves for connect-4/Toot-Otto
    - It is programmed using Trunk- a RUST frontend framework
- Backend consists of logic regarding computer moves
    - Connect 4 computer moves are determined by a minimax algorithm
    - Toot Otto computer moves are determined by a pseudo minimax algorithm for easy difficulty or by ChatGPT for hard difficulty
    - Backend was programmed using Rocket.rs due to its ease of use
- The division of the application into a front and backend ensured components were decoupled
    - This enabled backend and front logic to be tested separately
    - This also enabled developers to work in parallel without much version control conflict

### **SYSTEM LIMITATIONS**
- The Toot-Otto computer minimax algorithm has a random component to it
    - From our research, this also seemed to be present in the given MEAN stack repo
    - The computer chooses a T or O token at random during its moves
- Consequently, the algorithm sometime spits out random moves instead of trying to win
- GPT assisted hard mode also faces the same problem in that GPT does not respond properly 40% of the time.
    - In this case we have to rely on the flawed minimax algorithm for a computer move.

### **DESCRIPTION OF REMAINING MEAN STACK CODE**

### **USER MANUAL**

### **INSTALLATION**
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
