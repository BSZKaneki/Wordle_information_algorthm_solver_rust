use ahash::AHashMap;
use rand::prelude::IndexedRandom;
use std::collections::HashSet;
use std::io::{self};

struct WordleGame {
    target_word: String,
    correct_gussed_characters: Vec<(char, i32)>,
    attempts: usize,
    max_attempts: usize,
}
impl WordleGame {
    // fn open_file(path: &str) -> Result<Vec<String>, io::Error> {
    //     let file = File::open(path)?;
    //     let mut reader = BufReader::new(file);
    //     let mut buffer = Vec::new();
    //     reader.read_to_end(&mut buffer)?;

    //     let words: Vec<String> = buffer
    //         .split(|&byte| byte == b'\n')
    //         .filter_map(|line| String::from_utf8(line.to_vec()).ok())
    //         .collect();

    //     Ok(words)
    // }
    fn random_world() -> Result<String, io::Error> {
        const WORD_LIST: &str = include_str!("possible_anwsers.txt");
        let words: Vec<String> = WORD_LIST
            .lines()
            .filter_map(|line| line.trim().to_string().into())
            .collect();

        if words.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No words available",
            ));
        }

        let mut rng = rand::rngs::ThreadRng::default();
        let random_word = words.choose(&mut rng).expect("No words available");

        Ok(random_word.clone())
    }
    fn new(target_word: String, max_attempts: usize) -> Self {
        WordleGame {
            target_word,
            attempts: 0,
            max_attempts,
            correct_gussed_characters: vec![], // Initialize with 5 dots for each character
        }
    }
    fn entrohpy_allgorithm(&self) -> Result<(String, f64), io::Error> {
        const WORD_LIST: &str = include_str!("wordle_possibles.txt");
        let words: Vec<String> = WORD_LIST
            .lines()
            .filter_map(|line| line.trim().to_string().into())
            .collect();

        if words.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No words available",
            ));
        };
        let total_words = words.len() as f64;
        let mut biggest_entropy_word: String = String::new();
        let mut pattern_counts: AHashMap<String, usize> = AHashMap::new();
        let mut expected_information_gain = 0.0;

        for word in &words {
            let mut pattern = String::new();
            for (i, c) in word.chars().enumerate() {
                if word.chars().nth(i).unwrap() == c {
                    pattern.push('1'); // Correct position
                } else if self.target_word.contains(c) {
                    pattern.push('2'); // Wrong position
                } else {
                    pattern.push('0'); // Not in the word
                }
            }
            *pattern_counts.entry(pattern).or_insert(0) += 1;
            let mut entropy = 0.0;
            for &count in pattern_counts.values() {
                let probability = count as f64 / total_words;
                if probability > 0.0 {
                    entropy -= probability * probability.log2();
                }
            }
            if entropy > expected_information_gain {
                expected_information_gain = entropy;
                biggest_entropy_word = word.clone();
            }
        }

        Ok((biggest_entropy_word, expected_information_gain))
    }
        fn guess(&mut self, guessed_word: &str) -> Result<String, String> {
            // Create a set of already guessed characters (uppercase)
        let correct_guessed_set: HashSet<char> = self
            .correct_gussed_characters
            .iter()
            .map(|&(c, _)| c.to_ascii_uppercase())
            .collect();

        if guessed_word.len() != 5 {
            return Err("Please enter a valid 5-letter word.".to_string());
        }

        if guessed_word == self.target_word {
            return Ok(format!(
                "Congratulations! You've guessed the word: {}",
                self.target_word
            ));
        }

        self.attempts += 1;
        if self.attempts >= self.max_attempts {
            return Err(format!(
                "Sorry, you've used all attempts. The word was: {}",
                self.target_word
            ));
        }

        let mut correct_gussed_characters = String::with_capacity(5);
        let guessed_chars: Vec<char> = guessed_word.chars().collect();
        let target_chars: Vec<char> = self.target_word.chars().collect();

        for i in 0..5 {
            let c = guessed_chars[i];
            let target_char = target_chars[i];
            let uppercase_c = c.to_ascii_uppercase();

            if c == target_char {
                if !correct_guessed_set.contains(&uppercase_c) {
                    self.correct_gussed_characters
                        .push((uppercase_c, i as i32 + 1));
                }
                correct_gussed_characters.push(uppercase_c);
            } else if self.target_word.contains(c) {
                if !correct_guessed_set.contains(&uppercase_c) {
                    self.correct_gussed_characters.push((uppercase_c, 0));
                }
                correct_gussed_characters.push(c.to_ascii_lowercase()); // Differentiate from correct position
            } else {
                correct_gussed_characters.push('.');
            }
        }

        println!("{:?} ", self.correct_gussed_characters);
        Ok(format!("Correct characters: {}", correct_gussed_characters))
    }

    fn auto_game(&mut self) -> Result<(), String> {
        while self.attempts < self.max_attempts {
            let mut guessed_word_and_entropy = self.entrohpy_allgorithm();
            print!(
                "{:?} is the best guess word with entropy: ",
                guessed_word_and_entropy
            );
            let mut guessed_word = guessed_word_and_entropy.unwrap().0;
            println!(
                "Attempt {}: Please guess a 5-letter word:",
                self.attempts + 1
            );
            guessed_word = guessed_word.trim().to_string(); // Remove whitespace and newline characters

            match self.guess(&guessed_word) {
                Ok(message) => {
                    println!("{}", message);
                    if guessed_word == self.target_word {
                        return Ok(());
                    }
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
        Err(format!(
            "Sorry, you've used all attempts. The word was: {}",
            self.target_word
        ))
    }
}

/* Playable worldle game function


fn wordle_game() -> io::Result<()> {
    let file = File::open("D:\\programing\\rust\\Lerning\\game\\src\\possible_words.txt")?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let words: Vec<String> = buffer
        .split(|&byte| byte == b'\n')
        .filter_map(|line| String::from_utf8(line.to_vec()).ok())
        .collect();

    let mut rng = rand::thread_rng();
    let random_word = words.choose(&mut rng).expect("No words available");

    let mut attempts = 0;
    let max_attempts = 6;
    let mut guessed_word = String::new();
    let target_word = random_word.trim();
    println!("{} is the target word", target_word);

    while attempts < max_attempts {
        let mut gussed_characters = String::new();
        println!("Attempt {}: Please guess a 5-letter word:", attempts + 1);
        io::stdin().read_line(&mut guessed_word)?;
        guessed_word = guessed_word.trim().to_string(); // Remove whitespace and newline characters

        if guessed_word.len() != 5 {
            println!("Please enter a valid 5-letter word.");
            println!("{}",guessed_word.len());
            guessed_word.clear(); // Clear the guessed word for the next attempt
            continue;
        }

        if guessed_word == target_word {
            println!("Congratulations! You've guessed the word: {}", target_word);
            return Ok(());
        } else {
            println!("Incorrect guess. Try again.");
            attempts += 1;
            for (i, c) in guessed_word.chars().enumerate() {
                let target_char = target_word.chars().nth(i).unwrap();
                if c == target_char {
                    gussed_characters.insert(i, c.to_uppercase().next().unwrap());

                } else if target_word.contains(c) {
                    gussed_characters.insert(i, c);

                } else {
                    gussed_characters.insert_str(i,". ");
                }
            }
            println!("{} ", gussed_characters);

            guessed_word.clear();

        }
    }
    println!("Sorry, you've used all attempts. The word was: {}", target_word);

    Ok(())
}
*/

fn main() {
    let Random_word = WordleGame::random_world().expect("Failed to get a random word");
    let mut game = WordleGame::new(Random_word, 6);
    println!(
        "Welcome to Wordle! You have {} attempts to guess the 5-letter word.",
        game.max_attempts
    );

    // let entropy:(String, f64)     = game.entrohpy_allgorithm().expect("Failed to calculate entropy word");
    // print!("Entropy word is: {:?}", entropy);

    let auto_game_result = game.auto_game();
    match auto_game_result {
        Ok(_) => println!("Game completed successfully!"),
        Err(e) => println!("Game ended with error: {}", e),
    }

    // loop {
    //     let mut guessed_word = String::new();
    //     println!("Please enter your guess:");
    //     io::stdin().read_line(&mut guessed_word).expect("Failed to read line");
    //     guessed_word = guessed_word.trim().to_string(); // Remove whitespace and newline characters

    //     match game.guess(&guessed_word) {
    //         Ok(message) => {
    //             println!("{}", message);
    //             if guessed_word == game.target_word {
    //                 break; // Exit the loop if the word is guessed correctly
    //             }
    //         },
    //         Err(error) => {
    //             println!("{}", error);
    //         }
    //     }
    // }
}
