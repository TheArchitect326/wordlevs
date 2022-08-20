use clearscreen::clear;
use colored::{ColoredString, Colorize};
use rpassword::read_password;
use std::collections::HashSet;
use std::fmt;
use text_io::read;

pub fn play(allowed: &HashSet<String>) {
    let mut guesses: Vec<Action> = Vec::new();
    let mut hidden;
    let mut guess: String;

    for _ in 1..=6 {
        clear().expect("failed to clear screen");

        for action in &guesses {
            println!("{}", action)
        }

        println!("Hider's turn to play.");
        println!("Please enter a word.");

        loop {
            hidden = read_password().unwrap();

            if !allowed.contains(&hidden) {
                println!("Word not in game dictionary");
                println!("Please enter a word.");
                continue;
            }

            if !matches_history(&hidden, &guesses) {
                println!("Word does not match know information");
                println!("Please enter a word.");
                continue;
            }

            break;
        }

        println!("Guesser's turn to play.");
        println!("Please enter a word.");

        loop {
            guess = read!("{}");

            if !allowed.contains(&guess) {
                println!("Word not in game dictionary");
                println!("Please enter a word.");
                continue;
            }

            let a = Action {
                word: guess.chars().collect(),
                mask: Info::compute(&hidden, &guess),
            };
            guesses.push(a);
            break;
        }

        if guess == hidden {
            println!("GAMEOVER");
            println!("The word has been guessed correctly!!!");
            return;
        }
    }

    println!("GAMEOVER");
    println!("The word was sucsessfully hidden!!!")
}

fn matches_history(hidden: &String, guesses: &Vec<Action>) -> bool {
    for action in guesses.iter() {
        let word: String = (&action.word).into_iter().collect();
        if Info::compute(hidden, word.as_str()) != action.mask {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Info {
    // Green
    Correct,
    // Yellow
    Present,
    // Grey
    Missing,
}

impl Info {
    pub fn compute(hidden: &str, guess: &str) -> [Self; 5] {
        assert_eq!(hidden.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut mask = [Info::Missing; 5];
        // Array indexed by lowercase ascii letters
        let mut present = [0u8; 26];

        // Find all correct letters
        for ((hidden, guess), m) in hidden.chars().zip(guess.chars()).zip(mask.iter_mut()) {
            if hidden == guess {
                *m = Info::Correct
            } else {
                // If the letter does not match, count it as misplaced
                present[(hidden.to_string().as_bytes()[0] - b'a') as usize] += 1;
            }
        }
        // Check all of the non matching letters if they are misplaced
        for (guess, m) in guess.chars().zip(mask.iter_mut()) {
            // If the letter was guessed wrong and the same letter was counted as misplaced
            if *m == Info::Missing && present[(guess.to_string().as_bytes()[0] - b'a') as usize] > 0
            {
                *m = Info::Present;
                present[(guess.to_string().as_bytes()[0] - b'a') as usize] -= 1;
            }
        }
        mask
    }
}

struct Action {
    word: Vec<char>,
    mask: [Info; 5],
}

impl Action {
    fn colourise(&self, i: usize) -> ColoredString {
        match self.mask[i] {
            Info::Correct => self.word[i].to_string().as_str().green(),
            Info::Present => self.word[i].to_string().as_str().yellow(),
            Info::Missing => self.word[i].to_string().as_str().white(),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            format!("{}", self.colourise(0 as usize)),
            format!("{}", self.colourise(1 as usize)),
            format!("{}", self.colourise(2 as usize)),
            format!("{}", self.colourise(3 as usize)),
            format!("{}", self.colourise(4 as usize)),
        )
    }
}
