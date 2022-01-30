/// Some(true) => letter in correct position (green)
///
/// Some(false) => letter in incorrect position (yellow)
///
/// None => letter not in word (normal)
pub type LetterCorrect = Option<bool>;

/// Struct to represent the state of the game
pub struct GameState {
    /// the secret word
    pub word: String,
    /// array of past guesses (None represents no guess yet)
    pub past_guesses: [Option<String>; 5],
    /// the current guess
    pub current_guess: String,
}

impl GameState {
    /// Create a new GameState with the given secret word
    pub fn new(word: String) -> GameState {
        GameState {
            word: word.to_uppercase(),
            past_guesses: [None, None, None, None, None],
            current_guess: String::from(""),
        }
    }

    /// Add a guess to the game
    /// Moves current guess to past guesses and puts the new guess in current guess
    pub fn add_guess(&mut self, guess: String) -> () {
        let first_none = self.past_guesses.iter_mut().find(|x| x.is_none());

        if let Some(item) = first_none {
            if self.current_guess != "" {
                item.replace(self.current_guess.clone());
            }
            self.current_guess = guess.to_uppercase();
        }
    }

    /// Validate the current guess
    /// Returns an array representing the correctness of each letter
    pub fn validate_guess(&self, guess: &String) -> [LetterCorrect; 5] {
        let mut result = [Some(true); 5];

        for (i, (w, g)) in self.word.chars().zip(guess.chars()).enumerate() {
            result[i] = if w == g {
                Some(true)
            } else if self.word.chars().any(|c| c == g) {
                Some(false)
            } else {
                None
            };
        }

        result
    }

    /// Validate the current guess
    pub fn validate_current(&self) -> [LetterCorrect; 5] {
        self.validate_guess(&self.current_guess)
    }

    /// Check if the game is won
    pub fn win(&self) -> bool {
        self.current_guess == self.word
            || self
                .past_guesses
                .iter()
                .any(|w| w.is_some() && *w.as_ref().unwrap() == self.word)
    }

    /// Check if the game is lost
    pub fn lose(&self) -> bool {
        !self.win() && self.past_guesses.iter().all(|x| x.is_some())
    }

    /// Check if the game is over (won or lost)
    pub fn game_over(&self) -> bool {
        self.win() || self.lose()
    }
}
