fn main() {
    let s = String::from("hello world 1 2 3");
    let word = first_word(&s); // index of where the first blank space is
    let hello = &s[0..word]; // hello
    println!("{hello}");
    let rest = &s[word + 1..]; // world 1 2 3
    println!("{rest}");
    let test_end = &s[0..4];
    println!("{test_end}");
    let first_word_remade = first_word_remade(&s);
    println!("{first_word_remade}");
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_remade(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
