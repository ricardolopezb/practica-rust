pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len  > digits.len() {
        return Vec::new();
    }
    
    let mut vector = Vec::new();

    if len == 0 {
        for _ in 0..=digits.len() {
            vector.push("".to_string());
        }
        return vector;
    }

    let chr_array: Vec<char> = digits.chars().collect();
    for w in chr_array.windows(len) {
        vector.push(String::from_iter(w));
    }
    vector

}
