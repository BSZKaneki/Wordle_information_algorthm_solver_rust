use fantoccini::{ClientBuilder, Locator};
use std::time::Duration;
use tokio::time::sleep;

use crate::wordle_game;
use wordle_game::{CharGuess, WordleGame};

pub async fn run_wordle_bot() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = WordleGame::new(6);
    let mut client = ClientBuilder::native()
        .connect("http://localhost:50216")
        .await?;

    client
        .goto("https://www.nytimes.com/games/wordle/index.html")
        .await?;

    sleep(Duration::from_secs(2)).await;

    let buttons = client
        .find_all(Locator::Css(".fides-accept-all-button"))
        .await?;
    for b in buttons {
        if b.is_displayed().await? {
            b.click().await?;
            println!("Clicked visible cookie button ✅");
            break;
        }
    }

    sleep(Duration::from_secs(2)).await;

    let play_buttons = client
        .find_all(Locator::Css("[data-testid='Play']"))
        .await?;

    println!("Found {} play buttons", play_buttons.len());

    for b in play_buttons {
        if b.is_displayed().await? {
            b.click().await?;
            println!("Clicked visible play button ✅");
            break;
        }
    }

    sleep(Duration::from_secs(2)).await;

    let close_buttons = client
        .find_all(Locator::Css("[data-testid='icon-close']"))
        .await?;

    for b in close_buttons {
        if b.is_displayed().await? {
            b.click().await?;
            println!("Clicked close icon ✅");
            break;
        }
    }

    for row_index in 0..game.max_attempts {
        println!("Starting row {}", row_index);

        sleep(Duration::from_secs(3)).await;

        let (guess_word, entropy) = game.entrohpy_allgorithm()?; // propagate error
        println!(
            "{:?} is the best guess word with entropy: {}",
            guess_word, entropy
        );

        if let Err(e) = input_word(&mut client, &guess_word).await {
            println!("Error inputting word: {}", e);
            break;
        }

        let row_result =
            match word_results_from_row(&mut client, row_index, guess_word.clone()).await {
                Ok(res) => res,
                Err(e) => {
                    println!("Error getting row results: {}", e);
                    break;
                }
            };

        for fb in row_result.iter() {
            let guess = CharGuess {
                c: guess_word.chars().nth(fb.position).unwrap() as u8,
                feedback: fb.feedback,
                position: fb.position,
            };

            let is_duplicate = game.correct_gussed_characters.iter().any(|g| {
                g.c == guess.c && g.feedback == guess.feedback && g.position == guess.position
            });

            if !is_duplicate {
                game.correct_gussed_characters.push(guess);
            }
        }

        let display_str: String = row_result
            .iter()
            .map(|fb| {
                let c = guess_word.chars().nth(fb.position).unwrap();
                match fb.feedback {
                    2 => c.to_ascii_uppercase(),
                    1 => c.to_ascii_lowercase(),
                    _ => '.',
                }
            })
            .collect();

        println!("Row {} feedback: {}", row_index, display_str);
    }

    Ok(())
}

fn input_word(
    client: &mut fantoccini::Client,
    word: &str,
) -> impl std::future::Future<Output = Result<(), fantoccini::error::CmdError>> {
    let body = client.find(Locator::Css("body"));
    let word = word.to_string();
    async move {
        let body = body.await?;
        body.send_keys(&word).await?;
        body.send_keys("\u{E007}").await?; 
        Ok(())
    }
}

fn word_results_from_row(
    client: &mut fantoccini::Client,
    row_index: usize,
    guesed_word: String,
) -> impl std::future::Future<Output = Result<Vec<CharGuess>, fantoccini::error::CmdError>> + '_ {
    let row_selector = format!(
        ".Row-module_row__pwpBq[aria-label='Row {}'] .Tile-module_tile__UWEHN",
        row_index + 1
    );

    async move {
        let tiles = client.find_all(Locator::Css(&row_selector)).await?;
        let mut char_guesses = Vec::new();

        for (position, tile) in tiles.iter().enumerate() {
            sleep(Duration::from_millis(3000)).await;
            let letter = guesed_word
                .chars()
                .nth(position)
                .unwrap_or_default()
                .to_string();
            let state = tile.attr("data-state").await?.unwrap_or_default();

            let feedback = match state.as_str() {
                "correct" => 2, 
                "present" => 1,
                "absent" => 0,  
                _ => 0,         
            };
            let c = if let Some(ch) = letter.chars().next() {
                ch as u8
            } else {
                0 
            };

            char_guesses.push(CharGuess {
                c,
                feedback,
                position,
            });
        }

        Ok(char_guesses)
    }
}
#[tokio::test]
async fn test_wordle_bot_runs() {
    let result = run_wordle_bot().await;
    assert!(result.is_ok(), "Bot failed: {:?}", result);
}
