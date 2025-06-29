use std::io::{self, Write};

fn get_user_input() -> String {
    print!("Enter your input: ");
    let _ = io::stdout().flush();
    let mut usr_input = String::new();
    io::stdin()
        .read_line(&mut usr_input)
        .expect("Failed to Read Line");
    usr_input.trim_end().to_string()
}

fn analyze_string(sentence: &str) -> (usize, usize) {
    (
        sentence.trim().bytes().count(),
        sentence.trim().char_indices().count(),
    )
}

fn create_signature(sentence: String) -> String {
    format!("{} - RUST POWER", sentence.to_uppercase())
}
fn main() {
    let user_text = get_user_input();
    let result = analyze_string(&user_text);
    println!(
        "Byte length: {:0} Character count: {:1}",
        result.0, result.1
    );
    println!("Original text: {}", user_text);
    println!("Signature: {}", create_signature(user_text.clone()));
    println!("Original text after signature creation {}", user_text);
}
