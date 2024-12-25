use std::io::stdin;

pub fn solution() {
    let mut s = String::new();
    let cin = stdin();
    cin.read_line(&mut s).unwrap();
    s.clear();
    cin.read_line(&mut s).unwrap();
    let mut a: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut count = 0;
    a.sort_by(|a, b| b.cmp(a));
    for i in 1..(a.len() + 1) {
        if a[0..i].iter().sum::<i32>() > a[i..].iter().sum::<i32>() {
            count = i;
            break;
        }
    }
    println!("{}", count);
}
