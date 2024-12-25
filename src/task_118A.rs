use std::io::stdin;

const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

pub fn solution() {
    let mut s = String::new();
    let cin = stdin();
    cin.read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let mut new_s = vec![];

    for c in s.to_lowercase().chars() {
        if !VOWELS.contains(&c) {
            new_s.push('.');
            new_s.push(c);
        }
    }
    println!("{}", new_s.iter().collect::<String>());
}
