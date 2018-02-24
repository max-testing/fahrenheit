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

        match measurement.trim() {
            "F" => {
                println!("current temperature: {} degrees {}", temperature, measurement);
                println!("when converted, that's {} degrees",( (temperature - 32) * 5/9));
                break;
            },
            "C" => {
                println!("current temperature: {} degrees {}", temperature, measurement);
                println!("when converted, that's {} degrees", ( (temperature * 9/5) + 32));
                break;
            },
            _ => continue,
        }
    }
}
