use std::io::{self};

// reactor
fn main() {
    println!("Hello, this is currency converter!");
    let mut amount: f32;
    let mut currency_choose: i32;

    loop {
        loop {
            println!("Enter the amount in USD: ");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();

            // there i need second loop for checking type
            match user_input.trim().parse::<f32>() {
                Ok(num) => {
                    amount = num;
                    break;
                }
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            }
        }

        loop {
            println!("Please choose the currency you want to convert to:");
            println!("1: PLN");
            println!("2: USD");
            println!("3: YEN");
            println!("Enter any other number to exit");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<i32>() {
                Ok(num) => {
                    currency_choose = num;
                    break;
                }
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            }
        }

        match currency_choose {
            1 => {
                let converted = amount * 4.54;
                println!("{} EUR converted to {} PLN", amount, converted);
            }
            2 => {
                let converted = amount * 1.13;
                println!("{} EUR converted to {} USD", amount, converted);
            }
            3 => {
                let converted = amount * 129.53;
                println!("{} EUR converted to {} YEN", amount, converted);
            }
            _ => println!("Exit"),
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
