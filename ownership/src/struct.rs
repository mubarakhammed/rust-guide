fn main() {
    let mut s = String::from("hello world");
    let heelo = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s);
    s.clear();
}

fn first_word(s: &String) -> &str {
    let byte = s.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return i & s[0..i];
        }
    }

    &s[..]
}
