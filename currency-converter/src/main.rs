use std::io::{self};

fn main() {
    loop {
        println!("Hello, this is currency converter!");

        let amount = loop {
            println!("Please enter the amount in EUR you want to convert:");
            let mut amount = String::new();
            io::stdin().read_line(&mut amount).unwrap();
            match amount.trim().parse::<f32>() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Please enter a valid number.");
                }
            }
        };

        let input = loop {
            println!("Please choose the currency you want to convert to:");
            println!("1: PLN");
            println!("2: USD");
            println!("3: YEN");
            println!("Enter any other number to exit");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim().parse::<i32>() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Please enter a valid number.");
                }
            }
        };

        match input {
            1 => {
                let converted = amount * 4.54; // Assume 1 EUR = 4.54 PLN
                println!("{} EUR is {} PLN", amount, converted);
            }
            2 => {
                let converted = amount * 1.13; // Assume 1 EUR = 1.13 USD
                println!("{} EUR is {} USD", amount, converted);
            }
            3 => {
                let converted = amount * 129.53; // Assume 1 EUR = 129.53 YEN
                println!("{} EUR is {} YEN", amount, converted);
            }
            _ => break,
        }

        let continue_conversion = loop {
            println!("Do you want to convert another currency? (yes/no)");
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();
            match answer.trim().to_lowercase().as_str() {
                "yes" => break true,
                "no" => break false,
                _ => {
                    println!("Please enter 'yes' or 'no'.");
                }
            }
        };

        if !continue_conversion {
            break;
        }
    }
}
