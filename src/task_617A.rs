use std::io::stdin;

pub fn solution() {
    let mut s = String::new();
    let cin = stdin();
    cin.read_line(&mut s).unwrap();
    let mut left: usize = s.trim().parse().unwrap();
    let mut count = 0;
    for i in vec![5, 4, 3, 2, 1] {
        count += left / i;
        left = left % i;
    }
    println!("{}", count);
}