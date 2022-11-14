use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    
    let mut word_char_freq: HashMap<char, u32> = HashMap::new();
    let mut set_of_anagrams: HashSet<&'a str> = HashSet::new();
    
    for c in word.chars() {
        word_char_freq.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }

    for word in possible_anagrams {
        let mut possible_anagram: HashMap<char, u32> = HashMap::new();
        for chr in word.chars(){
            possible_anagram.entry(chr).and_modify(|count| *count += 1).or_insert(1);
        }
        if is_compatible(&word_char_freq, &possible_anagram) {
            set_of_anagrams.insert(*word);
        }
        
    }
    set_of_anagrams

    

}

fn is_compatible(original_word_map: &HashMap<char, u32>, possible_word_map: &HashMap<char, u32>) -> bool{
    for key in possible_word_map.keys(){
        if possible_word_map.get(key) != None 
        && original_word_map.get(key) != None
        && possible_word_map.get(key).unwrap() > original_word_map.get(key).unwrap(){
            return false;
        }
    }
    return true;
}
