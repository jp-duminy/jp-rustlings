// TODO: Fix the compiler error.
fn main() {
    let mut x: i32 = 3;  // making x mutable allows it to be reassigned to 5 on line 6
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
