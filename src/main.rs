
mod wordle_game;

mod  datascrape;
use datascrape::run_wordle_bot;


fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = run_wordle_bot().await {
            eprintln!("Error running Wordle bot: {}", e);
        }
    });
    // let mut game = wordle_game::WordleGame::new(6);
    // println!(
    //     "Welcome to Wordle! You have {} attempts to guess the 5-letter word.",
    //     game.max_attempts
    // );

    // let auto_game_result = game.auto_game();
    // match auto_game_result {
    //     Ok(_) => println!("Game completed successfully!"),
    //     Err(e) => println!("Game ended with error: {}", e),
    // }
}
