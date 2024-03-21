use solutions::stack::maximum_nest_depth_of_parens::max_depth;

pub fn main() {
    let s = String::from("(1+(2*3)+((8)/4))+1");
    let depth = max_depth(s);
    println!("Max depth: {}", depth);
}
