use std::io::stdin;

fn get_num_from_stdin<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

pub fn solution() {
    let a = get_num_from_stdin::<i32>();
    let b = get_num_from_stdin::<i32>();
    let c = get_num_from_stdin::<i32>();
    println!(
        "{}",
        [
            a + b + c,
            a * b * c,
            a + b * c,
            a * b + c,
            a * (b + c),
            (a + b) * c,
        ]
        .iter()
        .max()
        .unwrap()
    )
}
