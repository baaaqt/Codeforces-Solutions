use std::{collections::LinkedList, io::stdin};

pub fn solution() {
    let mut s = String::new();
    let cin = stdin();
    cin.read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let mut word = LinkedList::from_iter(['h', 'e', 'l', 'l', 'o']);
    for c in s.chars() {
        if word.front().is_none() {
            println!("YES");
            return;
        }
        if word.front().unwrap() == &c {
            word.pop_front();
        }
    }
    if word.len() == 0 {
        println!("YES");
        return;
    }
    println!("NO");
}
