use std::io;

pub fn solution() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    io::stdin().read_line(&mut s1).unwrap();
    io::stdin().read_line(&mut s2).unwrap();

    let s1 = s1.to_lowercase();
    let s2 = s2.to_lowercase();

    for i in 0..s1.len() {
        let s1_i = s1.chars().nth(i).unwrap();
        let s2_i = s2.chars().nth(i).unwrap();
        if s1_i < s2_i {
            println!("-1");
            return;
        } else if s1_i > s2_i {
            println!("1");
            return;
        } else {
            continue;
        }
    }
    println!("0");
}