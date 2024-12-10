pub fn reverse(input: &str) -> String {
    let mut output = "".to_string();
    for i in 0..input.len() {
        let c = input.chars().nth(i);
        match c {
            Some(c) => output = format!("{}{}", c, output),
            None => continue,
        }
    }
    output
}
