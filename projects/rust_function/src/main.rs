fn main() {
    let y: bool = is_even(4);
    println!("the y is even : {y} ");
    let x = if y { 10 } else { 20 };
    println!("the vwalue of x is: {}",x);
}

fn is_even(x: i32) -> bool {
    if x % 2 == 0 {
        return true;
    }
    false
}
