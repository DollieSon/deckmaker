use crate::{deck::{deckitem::WordDeck, filemanager::{detect_deck_text, read_file, save_deck_to_file}}, terminal::terminal::terminal_deck};

mod deck;
mod terminal;
fn main() {
    let main_t = terminal_deck::new();
    main_t.start();
}
