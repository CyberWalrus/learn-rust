fn main() {
    let txt: &str = "abcde";

    println!("последний символ строки {}", txt.chars().last().unwrap())
}
