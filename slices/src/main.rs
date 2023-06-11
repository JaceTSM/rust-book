use std::io;

fn main() {
    println!("Please enter a sentence: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let s: &str = &user_input;
    let w: &str = first_word(s);
    println!("First Word: {}", w);
    print_stacked(s)
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


fn print_stacked(s: &str) {
    let bytes: &[u8] = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        let mut just = String::new();
        for _ in [0..i] {
            just.push(' ');
        }
        println!("{}{}", just, b)
    }
}
