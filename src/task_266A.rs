use std::io::stdin;

pub fn solution() {
    let mut s = String::new();
    let cin = stdin();
    cin.read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();
    s.clear();
    cin.read_line(&mut s).unwrap();
    let colors: Vec<char> = s.trim().chars().collect();

    let mut count = 0;
    for i in 0..n - 1 {
        if colors[i] == colors[i + 1] {
            count += 1;
        }
    }
    println!("{}", count);
}