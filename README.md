# Wordle Game

## Installation

This project is written in Rust, so you'll need to have Rust and Cargo installed on your system. You can download Rust from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone the repository and build the project:

```
git clone https://github.com/your-username/wordle-game.git
cd wordle-game
cargo build --release
```

## Usage

To run the Wordle game, use the following command:

```
cargo run --release
```


## API

The main API for the Wordle game is the `WordleGame` struct, which provides the following methods:

- `new(max_attempts: usize)`: Creates a new Wordle game instance with the specified maximum number of attempts.
- `guess(&mut self, guessed_word: &str)`: Processes a user's guess and returns the feedback as a string.
- `auto_game(&mut self)`: Runs the auto-game mode, automatically guessing the best word based on the entropy algorithm.

