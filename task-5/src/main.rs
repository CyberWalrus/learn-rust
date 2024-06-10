fn main() {
    let word: &str = "денasdfadsfь";

    let mut word_char = word.chars().last().unwrap();

    if word_char.to_string() == "ь" {
        word_char = word.chars().rev().nth(1).unwrap();
    }

    println!("последний символ {}", word_char);
}
