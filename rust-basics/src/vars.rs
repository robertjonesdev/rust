// Cariables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-stoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // define a constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //assign multiple vars
    let ( my_name, my_age ) = ("Brad", "37");
    println!("{} is {}", my_name, my_age);
}