use std::io;

fn main() {
    let parsed_to_number: u32 = get_user_input();

    let results = factorial_calculation(parsed_to_number);

    println!("");
    println!("{}!= {}", parsed_to_number, results);
}

fn get_user_input() -> u32 {
    loop {
        println!("Hello, please give an input: ");

        let mut input_string = String::new();

        io::stdin().read_line(&mut input_string).unwrap();

        let trimmed_input = input_string.trim();

        match trimmed_input.parse::<u32>() {
            Ok(i) => return i,
            Err(_) => {
                println!("Not a valid number. Please try again.");
                continue;
            }
        }
    }
}

fn factorial_calculation(n: u32) -> u32 {
    let mut results: u32 = 1;

    for i in (1..=n).rev() {
        results = results * i;
        print!("{}", i);

        if i > 1 {
            print!(" x ");
        }
    }

    results
}
