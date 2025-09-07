use std::string;

use crate::deck::{deckitem::WordDeck, textreader::{detect_deck_text, read_file}};

mod deck;
fn main() {
    let sample = WordDeck::new();
    sample.print_self();
    let words:Vec<String> = [String::from("HELLO"),String::from("WORLD"),String::from("POGGIES")].to_vec();
    let grok = WordDeck::from_words(&words);
    grok.print_self();
    
    let wordle:Vec<String> = [String::from("I"),String::from("HATE"),String::from("GRANNIES")].to_vec();
    let gorckle = WordDeck::from_words(&wordle);
    gorckle.print_self();

    let mut wordvec = Vec::<WordDeck>::new();
    wordvec.push(gorckle);
    wordvec.push(grok);
    let finality = WordDeck::from_word_deck(&wordvec);
    finality.print_self();


    // let word_list = read_file();
    // let dicky = WordDeck::from_words(&word_list);
    // dicky.print_stat();
    // println!("Hello, world!");

    let files = detect_deck_text();
    println!("files {:?}",files);

    for file in files {
        let string_list = read_file(&file);
        let decked = WordDeck::from_words(&string_list);
        println!("{}",file);
        decked.print_stat();
    }

}
