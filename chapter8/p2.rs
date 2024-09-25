use std::io;

fn to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut chars = word.chars();

    let mut my_str = String::new();

    // Check the first character
    if let Some(first_char) = chars.next() {
        if vowels.contains(&first_char) {
            // If the word starts with a vowel, add "hay" to the end
            my_str.push_str(word);
            my_str.push_str("-hay");
        } else {
            // If the word starts with a consonant, move the first character to the end and add "ay"
            let rest: String = chars.collect();
            my_str.push_str(&rest);
            my_str.push('-');
            my_str.push(first_char);
            my_str.push_str("ay");
        }
    }

    my_str
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let words = vec!["first", "apple", "string", "orange"];

    let pig_latin_words: Vec<String> = words.iter().map(|&word| to_pig_latin(word)).collect();

    for word in pig_latin_words {
        println!("{}", word);
    }

    println!("{}", to_pig_latin(&input.trim()));
}

