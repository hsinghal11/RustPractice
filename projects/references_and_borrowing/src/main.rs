mod dangle;
mod mutable_ref;
fn main() {
    let s1 = String::from("hello");

    let len1 = calculate_length(&s1);

    println!("The length of '{s1}' is {len1}.");

    let mut s1 = String::from("Hello");
    let len = mutable_ref::mutable_reference(&mut s1);
    println!("The length of '{s1}' is {len}.");

    //   let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2); // cannot borrow `s` as mutable more than once at a time

    //  let mut s = String::from("hello");

    // {
    //     let r1 = &mut s; // correct codde
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem  // ( first we are using immutble then we are using mutable )
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
