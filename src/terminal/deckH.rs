use std::{io, sync::Arc};

use crate::deck::deckitem::WordDeck;

pub struct Ideck{
    pub deck: WordDeck,
    pub is_saved : bool,
}


impl Ideck {
    pub fn new(deck:WordDeck) -> Self{
        return Ideck { deck: deck, is_saved: false };
    }
    // rename (returns the new name)
    pub fn rename(&mut self)-> String{
        let mut input = String::new();
        input = io::stdin().read_line(&mut input)
                .expect("Error Parsing Message")
                .to_string();
        self.deck.name = input.clone();
        return input;
    }
    // print details
    pub fn print_details(&self){
        self.deck.print_stat();
    }
    pub fn print_words(&self){
        self.deck.print_word();
    }
    // add word
    pub fn add_word(&mut self){
        loop{
            print!("Enter word to add (Empty to stop):");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
            .expect("Error Parsing");
            let word = input.trim();
            if word == "" {
                break;
            }
            println!("Word : {}", word);
            self.deck.add_word(word.to_string());
        }
    }
    // remove word
    pub fn remove_word(&mut self){
        self.deck.print_word();
        loop{
            print!("Enter Word")
        }
    }
    // save as dictionary text
}