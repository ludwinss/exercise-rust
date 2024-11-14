use std::io;

fn main() {
    let mut input_number = String::new();

    println!("insert a number to generate a fibo (from 1 to 30)");
    let _ = io::stdin().read_line(&mut input_number);

    let input_number = match input_number.trim().parse::<u16>() {
        Ok(number) => number,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    if !(input_number >= 1 && input_number <= 30) {
        println!("is too short or too long");
        return;
    }

    let fibo_generated = generate_fibonacci(&input_number);

    fibo_generated.iter().for_each(|x| print!("{}, ", x));
}

fn generate_fibonacci(number_to_generate: &u16) -> Vec<u16> {
    let mut fibo_vector: Vec<u16> = vec![0, 1];

    if *number_to_generate <= 2 {
        return fibo_vector;
    }

    for idx in 2..=(*number_to_generate - 1) as usize {
        let number_calculate = fibo_vector[idx - 1] + fibo_vector[idx - 2];
        fibo_vector.push(number_calculate);
    }

    fibo_vector
}
