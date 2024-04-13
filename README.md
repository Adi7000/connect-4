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

3) What design choices exist for the Interface components? - Color? Font? Dimensions of Windows? Rescale-ability? Scroll Bars?
For our Connect 4 web app, several design choices exist for interface components:
- Color: We choose colors for the board, pieces, background, and interface elements that are visually appealing and provide good contrast for readability. 
        - We made the board blue, and the tokens yellow and Red (matching the traditional Connect 4 game for familiarity).
        - These colors have high contrast and help with readability
        - We labelled the tokens with R and Y to help people with color blindness
- Font: We selected  a clear and legible font (Times New Roman) for text elements such as rules descriptions, menu tabs, and player tokens.
- Dimensions of Windows: The dimensions of the game window and board was determined to be 85% of screen height and width (on safari). 
    - This ensured that they are large enough to display the game elements clearly without cluttering the interface. 
    - But the default dimensions seemed to vary on other browsers
    - This element will need to be further considered if the game needs to be ported to other devices like mobiles.
- Scroll Bars: We included a scroll bar. 
    - This was because our connect-4 board was on the top of the page and toot otto board on bottom of page.
    - The scroll bar allowed each oard to be displayed maximally on the screen one at a time. 
    - The scroll bar also allowed the user to zoom onto the boards while also being able to navigate to different board parts.
    
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
- We have two project submissions (branch main and branch routing) where branch main is the best solution we produced.
    - We had trouble integrating the branches together due to version issues with yew
    - The main branch provides the core functionality where the difficulty can be selected, however the UI does not fully allow routing
    - The routing branch allows for more intuitive routing but does not allow difficulty selection

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
