use core::slice;

fn main() {
    //integers

    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    //i32 can take 32 bit of data while i64 can take 64 bit. The i and u are for signed or unsigned
    //range for i32 is 2147483647
    //range for i64 is 9223372036854775807

    let i: i32 = 2147483647;
    let j: i64 = 9223372036854775807;
    println!("32 bit is: {}", i);
    println!("64 bit is: {}", j);

    //float [Floating data types - numbers with fractional parts]
    //we have f32 and f64

    let pi: f32 = 3.14;
    println!("The value of pi is: {}", pi);

    //boolean

    let is_snowing: bool = true;
    println!("Is is snowing?: {}", is_snowing);

    //char - character type

    let letter: char = 'a';
    println!("First letter of the alpha is: {}", letter);

    //compound data types
    //arrays, tuples , slices and strings

    //1. Arrays

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Mango", "Orange", "Apple"];
    println!("The fruit array is: {:?} ", fruits);
    println!("{}", fruits[0]);

    //Tuples
    let human = ("Alice", 30, false);
    println!("Human tuple is:{:?}", human);

    //slices: contigency

    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);

    //Strings Vs String Slices (&str)
    //Strings are [growable, mutable, owned string type]

    //1. String - string memory allocation is done on the heap hence slow and not easy to access

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah");
    println!("Stone cold says: {}", stone_cold);

    //2. String slices - very good for memory efficiency. Stored on the stack. a reference to a push of a string stored somewhere.

    let string: String = String::from("Hello, World");
    let slice: &str = &string;
    println!("Slice value: {}", slice);
}
