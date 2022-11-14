pub fn nth(n: u32) -> u32 {
    (2..).filter(|x| (2..(x-1)).all(|i| x%i != 0)).nth(n as usize).unwrap()
}

