fn main() {
    //--------- Ownership rules ---------//
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        let s: String = String::from("hello");
    }

    let s1 = String::from("hello");
    //   let s2 = s1; //This is a move not a copy or shallow copy there this is an error
    let s2 = s1.clone(); //This is a deep copy
    println!("{}", s1);
}
