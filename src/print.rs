pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    println!("{} is from {}", "Brad", "Mass");

    println!("{0} is from {1} and {0} likes to {2}","Brad", "Mass", "code.");

    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}