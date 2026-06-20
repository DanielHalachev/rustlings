// TODO: Fix the compiler error.
fn main() {
    // let x: i32 = 3;
    // FIX: make the variable mutable
    let mut x: i32 = 3;

    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
