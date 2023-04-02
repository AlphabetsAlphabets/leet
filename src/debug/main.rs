use solutions::stack::valid_parenthesis::is_valid;

pub fn main() {
    let parens = "[()]".to_string();
    let result = is_valid(parens.clone());
    println!("Is '{}' valid? {}", parens, result);
}
