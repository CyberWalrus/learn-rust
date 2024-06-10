fn main() {
    let word1: &str = "abc";
    let word2: &str = "ade";

    if first_letters_match(word1, word2) {
        println!("Первые буквы слов совпадают.");
    } else {
        println!("Первые буквы слов не совпадают.");
    }
}

fn first_letters_match(word1: &str, word2: &str) -> bool {
    let first_letter_word1 = word1.chars().next();
    let first_letter_word2 = word2.chars().next();

    match (first_letter_word1, first_letter_word2) {
        (Some(char1), Some(char2)) => char1 == char2,
        _ => false,
    }
}
