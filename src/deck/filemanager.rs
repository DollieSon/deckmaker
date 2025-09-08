const DIR_NAME:&str = "deck_text";


use std::{fs::{self, File}, io::{BufRead, BufReader, BufWriter, Write}};

fn is_correct(word: &String)-> bool{
    let ch_checker = word.chars().all(|ch| ch.is_ascii_alphabetic());
    let len_cheker = word.len() >= 4;
    return ch_checker && len_cheker;
}
//name of file (must be in deck_text)
pub fn read_file(file_name:&String) -> Vec<String>{
    let mut res = Vec::<String>::new();
    let file_name = format!("{}/{}",DIR_NAME,file_name);
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    for (line_num, line) in reader.lines().enumerate(){
        let line = line.unwrap();
        // check if availableA?
        if(is_correct(&line) == false){
            // println!("Inv Word : {}", line);
            continue;
        }
        res.push(line);
    }
    return res;
}

pub fn detect_deck_text()-> Vec<String>{
    let mut found_files = Vec::new();
    let file_dir = fs::read_dir(DIR_NAME).unwrap();
    
    for e in file_dir{
        let e = e.unwrap();
        let path = e.path();
        //so many ifs !!! Yummers
        if path.is_file(){
            if let Some(ext) = path.extension(){
                if ext == "txt"{
                    if let Some(name)= path.file_name(){
                        if let Some(name_str) = name.to_str() {
                            found_files.push(name_str.to_string());
                        }
                    }
                }
            }
        }
    }
    return found_files;
}

pub fn save_deck_to_file(deck: &Vec<String>, deck_name: &String){
    let path = format!("{}/{}.txt",DIR_NAME,deck_name);
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);
    for word in deck{
        writer.write_all(format!("{}\n",word).as_bytes()).unwrap();
    }
    println!("saved deck to {}",path);
}