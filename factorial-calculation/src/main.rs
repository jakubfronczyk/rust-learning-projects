use std::io;

fn main() {
    println!("Hello, please give an input: ");

    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string).unwrap();

    let trimmed_input = input_string.trim();

    // refactor following rust best practices

    let parsed_to_number: u32 = trimmed_input.parse().expect("Not a valid number");

    let results = factorial_calculation(parsed_to_number);

    println!("");
    println!("{}! = {}", parsed_to_number, results);

    // 4! = 4 x 3 x 2 x 1 = 24
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
