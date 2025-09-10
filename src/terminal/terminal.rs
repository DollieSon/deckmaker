use crate::{deck::{deckitem::WordDeck, filemanager::{detect_deck_text, read_file}}, terminal::helperS::Ideck};

pub struct terminal_deck{
    deck_list: Vec<Ideck>
}

impl terminal_deck{
    pub fn new() -> Self{
        return terminal_deck { 
            deck_list: Vec::new()
         }
    }
    // you can only start once
    pub fn start(mut self){
        // print main menu       
        println!("---------Deck Maker---------");
        println!("[1] Select Deck");
        println!("[2] Create Deck");
        println!("[4] Delete Deck");
        println!("[5] Detect Decks");
        println!("[6] Exit");
    }

    fn detect_decks(&mut self){
        let detected = detect_deck_text();
        println!("deck text files detected: ");
        println!("{:?}",detected);
        let new_deck_list = Vec::new();
        for deck_name in detected{
            let file_content = read_file(&deck_name);
            let mut deck = Ideck::new(
                WordDeck::from_words(&file_content,deck_name)
            );
            deck.is_saved = true;
            self.deck_list.push(deck);
        }
        self.deck_list = new_deck_list;
    }
    
}