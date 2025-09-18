use crate::{deck::{deckitem::WordDeck, filemanager::{detect_deck_text, read_file, save_deck_to_file}}, terminal::terminal::TerminalDeck};

mod deck;
mod terminal;
fn main() {
    let main_t = TerminalDeck::new();
    main_t.start();
}
