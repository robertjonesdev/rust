/*
    Primitive Types -
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    Float f32, f64
    Booleon (bool)
    Characters (char)
    Tuples
    Arrays
*/

pub fn run(){
    //Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    let z: i64 = 4545454545;

    // Find max size
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // get boolean from expression
    let is_greater = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

    

}