pub fn loop_practice() {
    println!("this is a practice of loops");

    // let mut num = 0;
    //=======================================================================
    // Loop as a expression
    // ======================================================================
    // let x = loop {
    //     if num == 10{
    //         break 20;
    //     }
    //     num+=1;
    // };

    // println!("the value of x: {x}")
    // =======================================================================
    // LOOP Practice
    // =======================================================================
    // loop {
    //     println!("the value of num: {num}");
    //     num += 1;
    //     if num == 10 {
    //         break;
    //     }
    // }
    // =======================================================================
    // LOOP Labels
    // =======================================================================
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");
    // =======================================================================
    // while Loop
    // =======================================================================
    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // =======================================================================
    // Bad Practice While Loop
    // =======================================================================

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // =======================================================================
    // Good Practice For Loop
    // =======================================================================
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
