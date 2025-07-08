use std::collections::HashSet;
fn main() {
    let words_array = ["apple", "banana", "orange", "grape", "kiwi"];
    let mut mutable_word_list = words_array.iter().map(|e| e).collect::<Vec<_>>(); //let mut mutable_word_list = words_array.to_vec();
    println!("Original words_array: {:?}", mutable_word_list);
    println!("Original mutable_word_list: {:?}", mutable_word_list);
    mutable_word_list.push(&"grapefruit");
    mutable_word_list.push(&"strawberry");
    mutable_word_list.remove(0);
    println!("Modified mutable_word_list: {:?}", mutable_word_list);

    let indexed_phrases = words_array
        .iter()
        .enumerate()
        .map(|(index, e)| format!("Index {} : Word: {}", index, e))
        .collect::<Vec<String>>();
    println!("Indexed phrases: {:?}", indexed_phrases);

    let unique_lowercase_words: HashSet<_> = mutable_word_list
        .into_iter()
        .map(|e| e.to_lowercase())
        .collect();
    println!("Unique lowercase words: {:?}", unique_lowercase_words);
}
