// Vectors - Resizeable arrays.

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;
    numbers.push(5);
    numbers.push(6);

    numbers.pop();
    println!("{:?}", numbers);

    println!("Single value: {}", numbers[0]);
    
    // get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice from array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
    
}

