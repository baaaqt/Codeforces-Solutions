use std::io::stdin;

pub fn solution() {
    let mut s = String::new();
    let cin = stdin();
    cin.read_line(&mut s).unwrap();
    let nums: Vec<usize> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = nums[0];
    let k = nums[1];

    let n_odd = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
    let k_num: usize;
    if k <= n_odd {
        k_num = 2 * k - 1;
    } else {
        k_num = 2 * (k - n_odd);
    }
    println!("{}", k_num);
}

fn main() {
    solution();
}
