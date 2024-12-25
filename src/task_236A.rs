use std::io::stdin;

pub fn solution() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let mut symbols: Vec<bool> = vec!(false; 26);
    let mut uniques = 0;
    for c in s.chars() {
        let i = (c as u8 - 97) as usize;
        if !symbols[i] {
            symbols[i] = true;
            uniques += 1;
        }
        if uniques == 26 {
            break;
        }
    }

    if uniques % 2 == 0 {
        println!("CHAT WITH HER!");
    } else {
        println!("IGNORE HIM!");
    }
}