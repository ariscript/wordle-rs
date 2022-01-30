# wordle-rs

A small wordle clone built in rust.

> Note: There are **a lot** of weird words I've never even heard in the dictionary file.
> If anyone knows a better word list, please do create a PR,

## Installation

1. Clone the repository
2. Install rust through rustup if required
3. Open a terminal in the cloned directory and run `cargo build --release`

## Usage

Run the newly created executable.
If you know how to play [Wordle](https://www.powerlanguage.co.uk/wordle) online, you already know how to play this game.

### How to play

1. Type any 5-letter wordle
2. The console should update with the letters of your word in different colors.
    - Green: the letter is correct and in the right location.
    - Yellow: the letter is in the word, but in another location.
    - Normal: the letter is not in the word.
3. Using the new information, guess again.
4. If you can guess the word within 6 tries, you win!

## License

This repository is licensed under version 3 of the GNU General Public License, or at your option, any later version.
