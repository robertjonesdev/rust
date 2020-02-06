// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure
//      - Use when you need to modify your own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    println!("Length: {}", hello.len());

    // Push on a char
    hello.push('W');

    // Push on a string
    hello.push_str("orld");

    // Capacity in bytes
    println!("{}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(3, s.len());
    println!("{}", s);


    println!("{}", hello);
}