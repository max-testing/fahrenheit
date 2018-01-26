//fahrenheight to celsius: (f - 32) x 5/9
//celsius to fahrenheight: (c x 9/5) + 32

use std::io;

fn main() {
    println!("Convert temparatures!");
    
    loop{
        println!("Enter Fahrenheight:");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: u32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("current temperature: {}", temperature);
    
        let converted_temperature = (temperature - 32) * 5/9;

        println!("that's {} in celsius!", converted_temperature);
        break;
    }
}

