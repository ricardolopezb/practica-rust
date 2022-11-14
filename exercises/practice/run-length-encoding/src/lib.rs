

pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return source.into();
    }

    let mut res = String::new();
    let mut prev_char = source.chars().nth(0).unwrap();
    let mut count: u32 = 0;
    for (i, c) in source.chars().enumerate() {
        if c != prev_char{
            if count != 1 {
                res.push_str(&count.to_string());
            } 
            res.push(prev_char);
            count = 1;
            prev_char = c;
        } else {
            count += 1;
        }
    }
    if count != 1 {
        res.push_str(&count.to_string());
    } 
    res.push(prev_char);
    res
}

pub fn decode(source: &str) -> String {
    
    let mut result = String::new();
    if source.is_empty() {
        return source.into();
    }
    let mut times_to_write = String::new();

    for c in source.chars() {
        if c.is_numeric(){
            times_to_write.push(c);
        } else {
            if times_to_write.is_empty() {
                result.push(c);
            }  
            else {
                let count =times_to_write.as_str().parse().unwrap();
                for i in 0..count {
                    result.push(c);
                }
            }
            times_to_write = String::new();
        }

    }
    result

}

