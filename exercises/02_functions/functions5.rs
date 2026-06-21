// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num // putting return num * num; throws a warning (but not a compile error)
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
