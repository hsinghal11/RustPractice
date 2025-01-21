use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    // let mut age = 33;
    // age = 2;
    println!("guess the number !");

    let random = rand::thread_rng().gen_range(1..=100);

    println!("random genreated number is: {}", random);

    loop {
        println!("Please input your guess.: ");

        let mut guess: String = String::new();
        io::stdin() 
            .read_line(&mut guess) 
            .expect("Failed to read line: line->12");

        let mut guess: u32 = match guess.trim().parse() {
            Ok(num  ) => num,
            Err(err) => {
                println!("please enter the no. or valid input");
                println!("{err}");
                continue;
            }
        };

        println!("You guessed: {}", &mut guess);

        match guess.cmp(&random) {
            Ordering::Equal => {
                print!("Yow won the game");
                break;
            }
            Ordering::Greater => println!("entered no. is Greater"),
            Ordering::Less => println!(" entered no. is smaller"),
        }

        //rand = "0.8.5"
        /*
           MAJOR = 0. version when you make incompatible API changes.
           MINOR = 8. version when you add functionality in a backward compatible manner.
           PATCH = 5. version when you make backward compatible bug fixes.
        */
    }
}
