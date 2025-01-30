mod temp;

fn main() {
    match temp::convert_temp(100.0, &'c') {
        Ok((temp, scale)) => println!("Converted temperature: {:.2} {}", temp, scale),
        Err(err) => println!("Error: {}", err),
    }

    match temp::convert_temp(212.0, &'f') {
        Ok((temp, scale)) => println!("Converted temperature: {:.2} {}", temp, scale),
        Err(err) => println!("Error: {}", err),
    }

    match temp::convert_temp(100.0, &'x') {
        Ok((temp, scale)) => println!("Converted temperature: {:.2} {}", temp, scale),
        Err(err) => println!("Error: {}", err),
    }
}