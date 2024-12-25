use std::io::stdin;

pub fn solution() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let nums: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut a = nums[0];
    let mut b = nums[1];
    let mut years = 0;
    while a <= b {
        a *= 3;
        b *= 2;
        years += 1;
    }
    println!("{}", years);
}