use std::{collections::HashMap, io};

use crate::{deck::{deckitem::WordDeck, filemanager::{detect_deck_text, read_file}}, terminal::deckH::Ideck};

pub struct terminal_deck{
    deck_list: HashMap<String,Ideck>
}

impl terminal_deck{
    pub fn new() -> Self{
        return terminal_deck { 
            deck_list: HashMap::new()
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
        let new_deck_list = HashMap::new();
        for deck_name in detected{
            let file_content = read_file(&deck_name);
            let mut deck = Ideck::new(
                WordDeck::from_words(&file_content,deck_name.clone().to_string())
            );
            deck.is_saved = true;
            self.deck_list.insert(deck_name, deck);
        }
        self.deck_list = new_deck_list;
    }


    fn delete_deck(&mut self){
        //print all decknames
        let decknames:Vec<String> = self.deck_list.keys().map(|s|{s.clone()}).collect();
        println!("Available Decks: {:?}",decknames);
        //enter deckname
        println!("Enter the name of the deck you want to delete: ");
        let mut input = String::new();
        loop {
            input = io::stdin().read_line(&mut input)
            .expect("Error Parsing Message")
            .to_string();
            println!("Deleting {}",input);
            if self.remove_deck(&input){
                break;
            }
        }
    }

    fn remove_deck(&mut self, deck_name: &String) -> bool{
        match self.deck_list.remove(deck_name){
            Some(x) => {
                println!("removed {}",x.deck.name);
                return true;
            },
            None => {
                println!("{} does not exist",deck_name);
                return false;
            }
        }
    }

    fn select_deck(&mut self){
        //find decks
        //operations available:
        // rename decks
        // print deck details
        // add word 
        // remove word
        // delete deck
        // combine with another deck
        // save as dictionary text
    }
}