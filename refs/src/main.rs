use std::io;


fn main() {
    print_seperator();
    ascii_play();
    print_seperator();
    incr_play();
    print_seperator();
}


fn print_seperator() {
    println!("------------------------------------------")
}


fn ascii_play() {
    println!("Please input a word:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    // input = input.trim();
    ascii_capitalize(&mut input);
    println!("Capitalized input: {}", input);
}

fn string_to_vec(s: &str) -> Vec<char> {
    s.chars().collect::<Vec<_>>()
}

fn vec_to_string(v: &Vec<char>) -> String {
    v.iter().collect::<String>()
}

fn ascii_capitalize(s: &mut String) {
    let mut v = string_to_vec(&s);
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {s}");
    }
    *s = vec_to_string(&v)
}


fn incr_play() {
    let mut n = 1;
    incr(&mut n);
    println!("{n}");
}

fn incr(n: &mut i32) {
  *n += 1;
}
