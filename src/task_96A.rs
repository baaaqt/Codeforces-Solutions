use std::io::stdin;

pub fn solution() {
    let mut s = String::new();
    let cin = stdin();
    cin.read_line(&mut s).unwrap();
    s = s.trim().to_string();
    if s.find("1111111").is_some() || s.find("0000000").is_some() {
        println!("YES");
    } else {
        println!("NO");
    }
}
