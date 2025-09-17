use std::{collections::{HashMap, HashSet}, io::{self, Read, Write}};

use crate::{deck::{deckitem::WordDeck, filemanager::{detect_deck_text, read_file, save_deck_to_file}}, terminal::deckH::Ideck};

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
    fn get_input() -> String{
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .expect("Error Parsing Input..");
        let res = input.trim();
        return res.to_string();
    }

    pub fn start(mut self){
        // print main menu       
        println!("---------Deck Maker---------");
        'main: loop {
            let mut input = String::new();
            println!("[1] Select Deck");
            println!("[2] Create Deck");
            println!("[3] Delete Deck");
            println!("[4] Detect Decks");
            println!("[5] Exit");
            println!(":");
            io::stdin().read_line(&mut input)
                .expect("Error Parsing Message");
            let choice = input.trim(); // Remove newline and whitespace
            println!("Choose : {}",choice);
            match choice {
                "1" => {
                    self.select_deck();
                }
                "2" => {
                }
                "3" => {
                    self.delete_deck();
                }
                "4" => {
                    self.detect_decks();
                }
                "5" => {
                    break 'main;
                }
                _ => {
                    println!("Incorrect Input")
                }
            }
        }
    }

    fn detect_decks(&mut self){
        let detected = detect_deck_text();
        println!("deck text files detected: ");
        println!("{:?}",detected);
        let mut new_deck_list = HashMap::new();
        for deck_name in detected{
            let file_content = read_file(&deck_name);
            let mut deck = Ideck::new(
                WordDeck::from_words(&file_content,deck_name.clone().to_string())
            );
            deck.is_saved = true;
            new_deck_list.insert(deck_name.clone(), deck);
        }
        self.deck_list = new_deck_list;
    }

    // TODO: ADD A CANCEL BUTTON
    fn delete_deck(&mut self){
        //print all decknames
        let decknames:Vec<String> = self.deck_list.keys().map(|s|{s.clone()}).collect();
        loop {
            println!("Available Decks: {:?}",decknames);
            println!("Enter the name of the deck you want to delete: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Error Parsing Message");
            let deck_name = input.trim();
            println!("Deleting {}",deck_name);
            if self.remove_deck(&deck_name.to_string()){
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
        let decknames:Vec<String> = self.deck_list.keys().map(|s|{s.clone()}).collect();
        let mut name = String::new();
        loop {
            println!("Available Decks: {:?}",decknames);
            if decknames.len() == 0 {
                println!("Empty Decks Cannot Select, Returning....");
                return;
            }
            let mut input = String::new();
            println!("Enter Deck to select: ");
            io::stdin().read_line(&mut input)
            .expect("Error Parsing Message")
            .to_string();
            input = input.trim().to_string();
            if decknames.contains(&input){
                name = input;
                break;
            }
            
            println!("{} does not exist",input);
        }
        let mut deck = self.deck_list.get_mut(&name).unwrap();
        println!("Deck Chosen : {}",name);
        loop {
            println!("Opertations Available");
            println!("[1] Add Word");
            println!("[2] Remove Word");
            println!("[3] Print Deck Details");
            println!("[4] Rename Deck");
            println!("[5] Combine With Another Deck");
            println!("[6] Save as a dictionary text");
            println!("[7] Delete Deck");
            println!("[8] Exit");
            print!("Operation: ");
            io::stdout().flush();
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Error Parsing Message");
            input = input.trim().to_string();
            match input.as_str() {
                "1"=> {
                    deck.add_word();
                }
                "2"=> {
                    deck.remove_word();
                }
                "3"=> {
                    deck.print_details();
                    deck.print_words();
                }
                "4"=> {
                    let old_name = deck.deck.name.clone();
                    let new_name = deck.rename();
                    name = new_name;
                    println!("{} has been renamed to {} ", old_name,name);
                }
                "5"=> {
                    let pairs:Vec<&String> = decknames.iter().filter(|&s| s != &name).collect();
                    let mut pair_name= String::new();
                    'pair_pick:loop {
                        println!("Decks {:?}",pairs);
                        println!("Enter Name of Paired Deck");
                        let mut temp = String::new();
                        io::stdin().read_line(&mut temp)
                            .expect("Error Parsing Message");
                        temp = temp.trim().to_string();
                        if pairs.contains(&&temp){
                            pair_name = temp;
                            break 'pair_pick;
                        }
                    println!("Invalid Input");
                    }
                    drop(deck);
                    let pair_deck = self.deck_list.get(&pair_name).unwrap();
                    let dup_deck = self.deck_list.get(&name).unwrap();
                    let mut combined = WordDeck::from_word_deck(&vec![&pair_deck.deck,&dup_deck.deck]);
                    let com_name = pair_deck.deck.name.clone() + &dup_deck.deck.name;
                    combined.rename(com_name.clone());
                    self.deck_list.insert(com_name,Ideck::new(combined));
                    deck = self.deck_list.get_mut(&name).unwrap();
                }
                "6"=> {
                    save_deck_to_file(&deck.deck.get_words(), &name);
                }
                "7"=> {
                    self.remove_deck(&name);
                    return;
                }
                "8"=> {
                    return;
                }
                _ => {
                    println!("Invalid Operation");
                }
            }
        }
    }

    fn create_deck(&mut self){
        let mut deck_name = String::new();
        // ask deck name
        loop{
            println!("Enter Deck Name:");
            let input = Self::get_input();
            match input.as_str(){
                "" => {
                    println!("Deck Name cannot be empty");
                }
                _ => {
                    deck_name = input;
                    break;
                }
            }

        }
        // ask to combine?
        let mut deck_type = String::new();
        let mut deck = WordDeck::new();
        loop{
            println!("Start with certain Decks or Empty?");
            println!("(E) Empty");
            println!("(C) Custom");
            let input = Self::get_input();
            match input.to_uppercase().as_str(){
                "E"=> {
                    //cheating
                    break;
                }
                "C" => {
                    deck = self.create_custom_deck();
                }
                _ => {
                    println!("Incorrect Input");
                }
            }
        }
        deck.rename(deck_name);
        // add words
        // confirm
    }
    fn create_custom_deck(&self) -> WordDeck{
        let mut decknames= self.deck_list.keys().map(|s|{s.clone()}).collect::<HashSet<String>>();
        let mut selected_decks = HashSet::<String>::new();
        loop {
            println!("Available Decks: {:?}",decknames);
            if decknames.len() == 0 && selected_decks.len() == 0 {
                println!("Empty Decks Cannot Select, Returning Empty Deckl");
                return WordDeck::new();
            }
            let mut input = String::new();
            println!("Enter Deck to select (Epmpty to stop): ");
            io::stdin().read_line(&mut input)
            .expect("Error Parsing Message")
            .to_string();
            input = input.trim().to_string();
            if decknames.contains(&input){
                decknames.remove(&input);
                selected_decks.insert(input);
            }
            else if decknames.len() == 0 {
                println!("No more decks.. stoping..");
                break;
            }
            else if input.is_empty(){
                println!("Stopping..");
                break;
            }else {
                println!("{} does not exist as a Deck Name",input);
            }
        }
        let mut decks = Vec::<&WordDeck>::new();
        for names in selected_decks{
            let deck = self.deck_list.get(&names).unwrap();
            decks.push(&deck.deck);
        }
        return WordDeck::from_word_deck(&decks);
    }
}