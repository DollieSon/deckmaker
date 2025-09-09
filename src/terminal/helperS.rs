use crate::deck::deckitem::WordDeck;

pub struct Ideck{
    pub deck: WordDeck,
    pub is_saved : bool,
}


impl Ideck {
    pub fn new(deck:WordDeck) -> Self{
        return Ideck { deck: deck, is_saved: false };
    }
}