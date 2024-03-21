use pebbles_game_io::{GameState, DifficultyLevel};
use gstd::msg;

// Define the state() function
#[no_mangle]
extern "C" fn state() {
    // Create an instance of GameState with dummy values for testing
    let game_state = GameState {
        pebbles_count: 15, 
        max_pebbles_per_turn: 3, 
        pebbles_remaining: 10, 
        difficulty: DifficultyLevel::Easy, 
        first_player: pebbles_game_io::Player::User,  // Use the Player enum from pebbles_game_io
        winner: None, 
    };

    // Reply with the game state
    msg::reply(game_state, 0).expect("Failed to send game state");
}


