// /// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut new_code = code.replace(" ", "");
    if new_code.len() <= 1 || new_code.chars().any(|x| !(x.is_digit(10))){
        return false;
    }

   let mut whole_code: String = new_code.chars().filter(|c| c.is_ascii_digit()).rev().collect();

   let mut new_str = String::new();
   for (i, mut num) in whole_code.chars().enumerate() {
    if i % 2 != 0 {
        let double_num = num.to_digit(10).unwrap()*2;
        let x = char::from_digit(if double_num <= 9 {double_num} else {double_num-9}, 10).unwrap();
        num = x;
    }
    new_str.push(num);

    }
    new_str.chars().fold(0, |mut acc, c| acc + c.to_digit(10).unwrap()) % 10 == 0

}