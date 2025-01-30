fn count_vowels(s: &String) -> usize {
    s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

fn main() {
    let text = String::from("Ownership and Borrowing");
    let vowels = count_vowels(&text);
    println!("The number of vowels is: {}", vowels); // Output: 8
}
