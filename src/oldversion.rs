use ahash::AHashMap;
use rand::prelude::IndexedRandom;
use rayon::{prelude::*};
use std::io::{self};


pub struct WordleGame {
    // target_word: String,
    correct_gussed_characters: Vec<CharGuess>,
    attempts: usize,
    max_attempts: usize,
    words: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct CharGuess {
    c: u8,
    feedback: u8,    // Green, Yellow, Gray
    position: usize, // actual index of the guess
}

impl WordleGame {
    fn new(max_attempts: usize) -> Self {
        const WORD_LIST: &str = include_str!("wordle_possibles.txt");
        let words: Vec<String> = WORD_LIST
            .lines()
            .filter_map(|line| line.trim().to_string().into())
            .collect();
        let mut rng = rand::rngs::ThreadRng::default();
        let random_word = words.choose(&mut rng).expect("No words available");
        // let target_word = random_word.clone();
        WordleGame {
            // target_word,
            attempts: 0,
            max_attempts,
            correct_gussed_characters: vec![],
            words,
        }
    }
    

    fn is_word_valid(&self, word: &str) -> bool {
        for guess in &self.correct_gussed_characters {
            let c = guess.c as char;
            let feedback = guess.feedback;
            let pos = guess.position;

            match feedback {
                2 => {
                    // Green: character must be at this position
                    if word.chars().nth(pos).unwrap_or('_') != c {
                        return false;
                    }
                }
                1 => {
                    // Yellow: must contain the character, but not at this position
                    if !word.contains(c) || word.chars().nth(pos).unwrap_or('_') == c {
                        return false;
                    }
                }
                0 => {
                    // Gray: must NOT contain the character *unless* already guessed as green/yellow
                    let appeared_in_other_guess = self
                        .correct_gussed_characters
                        .iter()
                        .any(|g| g.c == guess.c && g.feedback != 0);

                    if !appeared_in_other_guess && word.contains(c) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }

    fn pattern_from_guess(&self, guess: &str, answer: &str) -> String {
        let mut pattern = vec![0; 5];
        let mut answer_chars: Vec<char> = answer.chars().collect();
        let guess_chars: Vec<char> = guess.chars().collect();

        // First pass: mark greens (2)
        for i in 0..5 {
            if guess_chars[i] == answer_chars[i] {
                pattern[i] = 2;
                answer_chars[i] = '_'; // Mark as used
            }
        }

        // Second pass: mark yellows (1)
        for i in 0..5 {
            if pattern[i] == 0 {
                if let Some(pos) = answer_chars.iter().position(|&c| c == guess_chars[i]) {
                    pattern[i] = 1;
                    answer_chars[pos] = '_'; // Mark as used
                }
            }
        }

        pattern
            .iter()
            .map(|&n| char::from_digit(n, 10).unwrap())
            .collect()
    }

    fn entrohpy_allgorithm(&self) -> Result<(String, f64), io::Error> {
        

        let posible_words: Vec<String> = self.words
            .iter()
            .filter(|&word| self.is_word_valid(word))
            .cloned()
            .collect();
        let total_words = posible_words.len();

        // Debug: Print remaining possible words
        println!("Remaining possible words: {}", posible_words.len());

        if total_words == 1 {
            return Ok((posible_words[0].clone(), 0.0));
        } else if total_words <= 20 {
            println!("Words: {:?}", posible_words);
        }

        let entropies: Vec<(String, f64)> = self
            .words
            .par_iter()
            .map(|word| {
                let mut frequency_map: AHashMap<String, usize> = AHashMap::new();
                for w in &posible_words {
                    let pattern = self.pattern_from_guess(word, w);
                    *frequency_map.entry(pattern).or_insert(0) += 1;
                }

                let entropy = frequency_map
                    .values()
                    .map(|&count| {
                        if total_words == 0 {
                            return 0.0;
                        }
                        let p = count as f64 / total_words as f64;
                        if p > 0.0 { -p * p.log2() } else { 0.0 }
                    })
                    .sum();

                (word.clone(), entropy)
            })
            .collect();

        // Debug: Show top 5 candidates
        let mut sorted_entropies = entropies.clone();
        sorted_entropies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        println!("Top 5 entropy words:");
        for (word, entropy) in sorted_entropies.iter().take(5) {
            println!("  {}: {:.4}", word, entropy);
        }

        let (best_word, best_entropy) = entropies
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .clone();

        println!("Best word: {}, Entropy: {}", best_word, best_entropy);
        Ok((best_word, best_entropy))
    }

    fn guess(&mut self, guessed_word: &str) -> Result<String, String> {
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

        let mut correct_gussed_characters_str = String::with_capacity(5);
        let guessed_chars: Vec<char> = guessed_word.chars().collect();
        let target_chars: Vec<char> = self.target_word.chars().collect();

        for i in 0..5 {
            let c = guessed_chars[i];
            let target_char = target_chars[i];
            let feedback = if c == target_char {
                2 // Green
            } else if self.target_word.contains(c) {
                1 // Yellow
            } else {
                0 // Gray
            };

            let guess = CharGuess {
                c: c as u8,
                feedback,
                position: i,
            };

            // Only insert if this exact guess doesn't already exist
            let is_duplicate = self.correct_gussed_characters.iter().any(|g| {
                g.c == guess.c && g.feedback == guess.feedback && g.position == guess.position
            });

            if !is_duplicate {
                self.correct_gussed_characters.push(guess);
            }

            // Build display string
            match feedback {
                2 => correct_gussed_characters_str.push(c.to_ascii_uppercase()),
                1 => correct_gussed_characters_str.push(c.to_ascii_lowercase()),
                _ => correct_gussed_characters_str.push('.'),
            }
        }

        Ok(format!(
            "Correct characters: {}",
            correct_gussed_characters_str
        ))
    }

    // fn auto_game(&mut self) -> Result<(), String> {
    //     while self.attempts < self.max_attempts {
    //         let guessed_word_and_entropy = self.entrohpy_allgorithm();
    //         print!(
    //             "{:?} is the best guess word with entropy: ",
    //             guessed_word_and_entropy
    //         );
    //         let mut guessed_word = guessed_word_and_entropy.unwrap().0;
    //         println!(
    //             "Attempt {}: Please guess a 5-letter word:",
    //             self.attempts + 1
    //         );
    //         guessed_word = guessed_word.to_string(); // Remove whitespace and newline characters

    //         match self.guess(&guessed_word) {
    //             Ok(message) => {
    //                 println!("{}", message);
    //                 if guessed_word == self.target_word {
    //                     return Ok(());
    //                 }
    //             }
    //             Err(error) => {
    //                 println!("{}", error);
    //             }
    //         }
    //     }
    //     Err(format!(
    //         "Sorry, you've used all attempts. The word was: {}",
    //         self.target_word
    //     ))
    // }
}