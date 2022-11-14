pub fn is_armstrong_number(num: u32) -> bool {
    
    let str_num = format!("{}", num);
    let exp = str_num.len();
    let mut count = 0;
    
    for i in str_num.chars(){
        count += i.to_digit(10).unwrap().pow(exp as u32);
    }
    count == num


}
