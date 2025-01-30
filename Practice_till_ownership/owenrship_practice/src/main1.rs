pub fn main() {
    let mut num = 42;
    let ref1 = &num; // Immutable reference
    let ref2 = &mut num; // Mutable reference (Fix this!)

    println!("ref1: {}", ref1);
    println!("ref2: {}", ref2);
}
