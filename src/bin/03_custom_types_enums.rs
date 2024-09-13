#![allow(dead_code)]
// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    use Number::*;

    // `enums` can be cast as integers.
    println!("zero is {}", Zero as i32);
    println!("one is {}", One as i32);

    use Color::*;
    println!("roses are #{:06x}", Red as i32);
    println!("violets are #{:06x}", Blue as i32);
}