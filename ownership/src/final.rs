fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: &String) -> usize {
    let length: usize = s.len();
    length
}
