use crate::hangman::error::{HangmanError, HangmanErrorKind};
use std::collections::HashSet;

/// DO NOT CHANGE THIS, you may use this as you see fit
/// The number of guesses allowed before the game ends.
pub const NUM_GUESSES_TOTAL : usize = 6;

/// Struct to handle a Hangman game and store all necessary game state.
/// Feel free to design this struct however you like, as long as you don't
/// modify any of the function signatures in the impl block.
/// TODO: design the Hangman struct.
#[derive(Debug, Default)]
pub struct Hangman {
    left_guess_num: usize,
    word: String,
    correct_guess: HashSet<char>,
    incorrect_guess: HashSet<char>,
    word_set: HashSet<char>,
}

/// Hangman impl block.
/// Your task is to finish all the functions in this block. You may add any
/// helper functions as you need. After you have finished all the functionality
/// below, feel free to run the program and play with the command line interface which should be 
/// fully functional assuming that your code passes all tests. See the README for instructions.
/// Note: Make sure not to change the function signatures below!!!
/// TODO: finish all the functions in the impl block.
impl Hangman {
    /// Instantiates a new Hangman game object for the given word.
    /// Returns an Ok(Hangman) if the word was valid.
    /// Returns an Err(HangmanError) if the word was an empty string or contained non-alpha chars.
    pub fn new(word: String) -> Result<Self, HangmanError> {
        match word.is_empty() || !word.chars().all(|c| c.is_alphabetic()) {
            true => Err(HangmanError::new(HangmanErrorKind::InvalidWord, "".to_string())),
            false => Ok(Hangman {
                    left_guess_num: NUM_GUESSES_TOTAL,
                    word: word.chars().map(|c| c.to_ascii_lowercase()).collect(),
                    correct_guess: HashSet::new(),
                    incorrect_guess: HashSet::new(),
                    word_set: word.chars().map(|c| c.to_ascii_lowercase()).collect(),
            }),
        }
    }

    /// Guesses a character in the hangman game and updates the game state
    /// Note: the guess is case INSENSITIVE (e.g., if the word is "abc," both 'A' and 'a' are correct guesses).
    /// Returns Ok(true) if the guess was valid and correct
    /// Returns Ok(false) if the guess was valid, but incorrect
    /// Returns a GameAlreadyOver error if the game was already finished before the guess
    /// Returns an InvalidCharacter error if the character is not alphabetic
    /// Returns an AlreadyGuessedCharacter error if the character was already guessed 
    /// (either correctly or incorrectly) in the game.
    pub fn guess(&mut self, c: char) -> Result<bool, HangmanError> {
        if self.left_guess_num == 0 || self.correct_guess.len() == self.word_set.len() {
            return Err(HangmanError::new(HangmanErrorKind::GameAlreadyOver, "".to_string()));
        }

        if !c.is_alphabetic() {
            return Err(HangmanError::new(HangmanErrorKind::InvalidCharacter, "".to_string()));
        }

        let l = c.to_ascii_lowercase();
        if self.correct_guess.contains(&l) || self.incorrect_guess.contains(&l) {
            return Err(HangmanError::new(HangmanErrorKind::AlreadyGuessedCharacter, "".to_string()));
        }

        if self.word_set.contains(&l) {
            self.correct_guess.insert(l);
            return Ok(true);
        }

        self.left_guess_num -= 1;
        self.incorrect_guess.insert(l);
        return Ok(false);
    }

    /// Returns a reference to the game word converted to lowercase.
    pub fn get_word(&self) -> &String {
        &self.word
    }

    /// Returns the number of guesses left before the guesser loses.
    pub fn get_num_guesses_left(&self) -> usize {
        self.left_guess_num
    }

    /// Returns a reference to a HashSet of all correct guessed characters.
    pub fn get_correct_guesses(&self) -> &std::collections::HashSet<char> {
        &self.correct_guess
    }

    /// Returns a reference to a HashSet of all incorrectly guessed characters.
    pub fn get_incorrect_guesses(&self) -> &std::collections::HashSet<char> {
        &self.incorrect_guess
    }

    /// Returns the result of the game.
    /// Returns Some(true) if the user guessed all the characters in the word without exceeding
    /// the allowed number of guesses.
    /// Returns Some(false) if the user made too many incorrect guesses.
    /// Otherwise, the game is in progress, so return None.
    pub fn get_game_result(&self) -> Option<bool> {
        if self.correct_guess.len() == self.word_set.len() {
            return Some(true);
        }

        if self.left_guess_num == 0 {
            return Some(false);
        }

        return None;
    }
}
