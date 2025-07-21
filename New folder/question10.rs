fn main() {
  let string_literal: &str = "I am a literal";
    let string_object: String = String::from(string_literal);
    println!("String Literal: {}", string_literal);
    println!("String Object: {}", string_object);
}