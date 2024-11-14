use std::io;

fn main() {
    let mut input_number: String = String::new();
    println!("inser a number to calculate factoria from 1 to 20");

    let _ = io::stdin().read_line(&mut input_number);

    let input_number = match input_number.trim().parse() {
        Ok(number) => number,
        Err(_) => 0,
    };

    if input_number >= 1 && input_number <= 20 {
        let factorial_number = calculate_factorial(&input_number);
        println!("the factorial is: {}", factorial_number);
    } else {
        println!("the number is too long ");
    }
}

fn calculate_factorial(input_number: &u64) -> u64 {
    if *input_number == 0 {
        return 1;
    }

    let mut idx: u64 = 1;
    let mut final_result: u64 = 1;

    while idx <= *input_number {
        final_result = final_result * idx;
        idx = idx + 1;
    }

    final_result
}
