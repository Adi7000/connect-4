### **MAJOR INNOVATIONS**
- Our Toot Otto game variant's API's hard mode computer moves are GPT assisted
- We utilized ChatGPT to get the code for how to communicate with GPT-4-Turbo API

### **RATIONALE FOR DESIGN DECISIONS**
1) What can we do on a computer that we can’t do on a printed board?
    A printed board allows creative freedom to the developers to put in effects as they please. For example, 
upon winning connect-4, the board is flooded with the winning player's color for special effects. In a real 
game, this would not be possible as there would not be suffiecient tokens nor would it be practicle for a human
to do so for effect. 
A computerized version also enables for the win checking algorithms to be robust whereas on a printed board, 
there is no guarantee that a human would detect reliably if the game has ended.

2) What is a computerized opponent? What are its objectives? (Remember, not everyone is an expert.)
- What characteristics should it possess? Do we need an opponent or opponents?
    A computerized opponent is when an oponenet in a multiplayer game is a computer algorithm. Ideally a
computerized opponent should come with multiple levels of difficulty to allow all types of users to enjoy
playing a game. For our game, this comes in the form of Easy and Hard difficulties. In our game we only need
one computerized opponent, however, in other games more computerized opponents may be necessary where more than
2 players compete at a time.

3) What design choices exist for the Interface components?
- Color? Font? Dimensions of Windows? Rescale-ability? Scroll Bars? ….

4) What does exception handling mean in a GUI system?
Exception handling in a GUI can invlve the following:
    - Detection: Monitoring for errors or unexpected situations during program execution. This includes checking return values from API calls to see if things are fine
    - Reporting: If errors were detected, it should log the error to a file or console for developers to fix it
    - Providing feedback: It should provide feedback to the user in case of illegal inputs and not crash 
    - Graceful Handling: Ensuring stability and responsiveness despite errors. The GUI should not crash during errors and illegal user inputs. In both cases, it should inform the user of the problems.
    - Recovery: GUI should attempt to restore application stability by reverting or prompting corrective action.
    
5) Do we require a command-line interface for debugging purposes????? (The answer is yes by the way – please explain why)
    - We do require a command line interface.
    - Developers might want to test out the computer algorithms (in our case the minimax algorithms) even before the front end is made
    - In that case, a simple CLI that prints out computer moves with a given 2D vector will come in handy
    - We have comething like this in our api_usage_example folder where we can get computer responses printed to the terminal in response to given 2D vectors


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
https://youtu.be/CmRCfi2TAD8