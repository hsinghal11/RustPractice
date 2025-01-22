// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello"); // the S will die in this scope so we cant give it to anyone else

//     &s
// }

// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }
