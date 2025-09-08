use std::collections::HashSet;

const ALPHABET_COUNT :usize = 26;
const ASCII_BASE:u8 = 65; // 'A'
pub struct WordDeck{
    pub deck: HashSet<String>,
    pub lettter_count: Vec<u32>
}

impl WordDeck{
    pub fn new() -> Self{
        return WordDeck { deck:
            HashSet::new(), 
            lettter_count: vec![0;ALPHABET_COUNT]
        };
    }
    //assumptions both box are the same len
    fn combine_letter_count(main_box: &mut Vec<u32>, sec_box: &Vec<u32>){
        for (ind,count) in sec_box.iter().enumerate(){
            main_box[ind] += *count;
        }
    }
    //from files or such
    // Assumptions Alphabet is English/Roman and ALL Capital Letters
    pub fn from_words(string_deck: &Vec<String>) -> Self{
        let mut res = Self::new();
        for word in string_deck{
            res.deck.insert(word.clone());
            //count chars
            let characters = word.chars()
            .map(|ch|{(ch.to_ascii_uppercase() as u8) - ASCII_BASE } as usize)
            .fold(vec![0;ALPHABET_COUNT], |mut vec,x|{
                vec[x]+=1;
                vec
            });
            //Combine them        
            Self::combine_letter_count(&mut res.lettter_count, &characters);
        }
        return res;
    }

    // for combining decks 
    // should we have duplicate checking? 
    pub fn from_word_deck(word_deck: &Vec<WordDeck>) -> Self{
        let mut res = Self::new();
        for deck in word_deck{
            res.deck.extend(deck.deck.iter().cloned());
            Self::combine_letter_count(&mut res.lettter_count, &deck.lettter_count);
        }
        return res;
    }

    pub fn print_self(&self){
        Self::print_stat(&self);
        for word in self.deck.iter(){
            println!("{}",word);
        }
    }

    pub fn print_stat(&self){
        println!("Word count: {} ", self.deck.len());
        println!("Letter Count: {} , {:?}", self.lettter_count.len(), self.lettter_count);
    }

    pub fn get_deck(&self) -> Vec<String> {
        let mut res:Vec<String> = self.deck.clone().into_iter().collect();
        res.sort();
        return res;
    }
}