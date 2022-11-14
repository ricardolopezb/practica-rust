pub fn twofer(name: &str) -> String {
    let name_to_return;
    if name.is_empty() {
        name_to_return = "you";
    } else {
        name_to_return = name;
    }
    return format!("One for {}, one for me.", name_to_return);
}
