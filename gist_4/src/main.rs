use std::io::{self, Write};
fn get_sentence() -> String {
    print!("Enter a sentence: ");
    let _ = io::stdout().flush();
    let mut usr_input = String::new();

    io::stdin()
        .read_line(&mut usr_input)
        .expect("Failed to Readline");
    usr_input
}

fn count_words(sentence: &str) -> usize {
    let words = sentence.split_whitespace();
    words.count()
}

fn reverse_words(sentence: String) -> String {
    sentence
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    let sentences = get_sentence();
    println!("Word count: {} ", count_words(&sentences),);
    print!("Original sentences: {}", sentences);
    println!("Reversed Sentence: {}", reverse_words(sentences.clone()));
}
