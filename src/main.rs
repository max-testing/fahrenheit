//fahrenheight to celsius: (f - 32) x 5/9
//celsius to fahrenheight: (c x 9/5) + 32

use std::io;

fn main() {
    println!("Convert temparatures!");
    
    loop{
        println!("Enter Number of Degrees:");
        
        let mut temperature = String::new();
        
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("in F or C?");
        
        let mut measurement = String::new();
        
        io::stdin().read_line(&mut measurement)
            .expect("Failed to read line");

        println!("current temperature: {} degrees {}", temperature, measurement);
        let converted_temperature: i32 = if measurement.trim() == "F" {
                (temperature - 32) * 5/9
            } else {
                (temperature * 9/5) + 32
            };

        println!("when converted, that's {} degrees", converted_temperature);
        break;
    }
}

