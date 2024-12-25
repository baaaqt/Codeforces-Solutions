use std::io;

pub fn solution() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let first_letter = s.chars().nth(0).unwrap();
    if first_letter.is_lowercase() {
        s.replace_range(..1, &first_letter.to_string().to_ascii_uppercase());
    }
    println!("{}", s);
}