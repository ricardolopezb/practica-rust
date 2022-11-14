// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    if magazine.len() < note.len(){
        return false;
    }

    let mut magazine_map: HashMap<&str, u32> = HashMap::new();
    let mut note_map = HashMap::new();

    for word in magazine.iter() {
        magazine_map.entry(word).and_modify(|count| *count += 1).or_insert(1);
    }

    for note_word in note.iter() {
        note_map.entry(note_word).and_modify(|count| *count += 1).or_insert(1);
    }

    for key in note_map.keys(){
        let mag_count = magazine_map.get(*key);
        if mag_count == None || mag_count.unwrap() < note_map.get(*key).unwrap(){
            return false;
        }
    }
    true
}
