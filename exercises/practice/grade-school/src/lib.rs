// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
use std::collections::HashMap;
pub struct School {
    grades: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School {grades: HashMap::new()}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.entry(grade)
        .and_modify(|studs| studs.push(String::from(student)))
        .or_insert(vec![String::from(student)]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut myGrades: Vec<u32> = self.grades.keys().map(|&x| x).collect();
        myGrades.sort();
        myGrades

    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if self.grades.contains_key(&grade) {
            let mut result = self.grades.get(&grade).unwrap().to_vec().clone();
            result.sort();
            return result;
        } else {
            return Vec::new();
        }
    }
}
