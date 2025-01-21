fn main() {
    let y: bool = is_even(4);
    println!("the y is even : {y} ");
}

fn is_even(x: i32) -> bool {
    if x % 2 == 0 {
        true
    } else {
        false
    }
}
