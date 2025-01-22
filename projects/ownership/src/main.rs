fn main() {
    let s1 = gives_ownership();
    let s2 = gives_ownership();
    take_ownership(s2);
    let s1 = takes_and_gives_back(s1);
    println!("this is s1: {s1},\n and this is s2");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn gives_ownership() -> String {
    String::from("this is from gives_ownership")
}

fn take_ownership(s: String) {
    println!("I have taken the the ownership of {s}");
}

fn takes_and_gives_back(s: String) -> String {
    println!("i have taken owernship of {s} and returning it");
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length) // returning (ownership, length)
}