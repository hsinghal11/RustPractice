// pub fn convert_temp(temp: f32, &s: &char) -> (f32, String){
//     if s == 'c'{
//         let temp = (temp * 9.0/5.0) + 32.0;
//         return (temp, String::from("f"));
//     }else if s == 'f' {
//         let temp = (temp - 32.0) * 5.0/9.0;
//         return (temp, String::from("c"));       
//     }else{
//         println!("degree of temp is wrong");
//         return (temp, String::from("wrong"));
//     }
// }

pub fn convert_temp(temp: f32, s: &char) -> Result<(f32, char), String> {
    if (*s == 'c') || (*s == 'C') {
        let converted_temp = (temp * 9.0 / 5.0) + 32.0;
        Ok((converted_temp, 'f')) // Return Fahrenheit
    } else if (*s == 'f') || (*s == 'F') {
        let converted_temp = (temp - 32.0) * 5.0 / 9.0;
        Ok((converted_temp, 'c')) // Return Celsius
    } else {
        Err(String::from("Invalid temperature scale. Use 'c' or 'f'."))
    }
}
