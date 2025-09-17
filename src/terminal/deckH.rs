use std::{io::{self, Write}, sync::Arc};

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
        io::stdin().read_line(&mut input)
                .expect("Error Parsing Message")
                .to_string();
        input = input.trim().to_string();
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
    fn get_word() -> String{
        let mut input = String::new();
            io::stdin().read_line(&mut input)
            .expect("Error Parsing");
        let word = input.trim();
        return word.to_string();
    }
    // add word
    pub fn add_word(&mut self){
        loop{
            print!("Enter word to add (Empty to stop):");
            io::stdout().flush();
            let mut word = Self::get_word();
            if word == "" {
                break;
            }
            word = word.to_ascii_uppercase();
            println!("Word : {}", word);
            self.deck.add_word(word.to_string());
        }
    }
    // remove word
    pub fn remove_word(&mut self){
        self.deck.print_word();
        loop{
            print!("Enter Word(empty to stop) :");
            let word = Self::get_word();
            if word == "" {
                break;
            }
            println!("Removing : {}", word);
            self.deck.remove_word(&word.to_string());
        }
    }

    pub fn get_word_vec(&self) -> Vec<String>{
        return self.deck.get_words();
    }
    // save as dictionary text
}