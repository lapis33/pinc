use crate::{drivers::uart::read_char, uart_print, uart_println};
use alloc::{string::String, vec::Vec};

const LOGO: &str = r#"
               /$$                    
              |__/                    
      /$$$$$$  /$$ /$$$$$$$   /$$$$$$$
     /$$__  $$| $$| $$__  $$ /$$_____/
    | $$  \ $$| $$| $$  \ $$| $$      
    | $$  | $$| $$| $$  | $$| $$      
    | $$$$$$$/| $$| $$  | $$|  $$$$$$$
    | $$____/ |__/|__/  |__/ \_______/
    | $$                              
    | $$                              
    |__/                             
"#;

pub async fn main() {
    uart_println!("{}", LOGO);

    loop {
        uart_print!("[root@pinc] ");

        let mut input = String::new();

        loop {
            let character = read_char();
            uart_print!("{}", character);

            match character {
                '\n' => break,
                other => input.push(other),
            }
        }

        let arguments: Vec<&str> = input.split_ascii_whitespace().collect();
    }
}
