fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("First word bound of {}: {}", s ,word.len());
    // Can't clear. slice exists
    // s.clear();
}

// & -reference, takes value not ownership
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
