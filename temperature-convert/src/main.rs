use std::{i16, io};

// cual es la diferencia entre str String

fn main() {
    let mut input_number = String::new();
    let mut input_type_grades = String::new();

    println!("insert a number of grades into from -32 to 1000");
    let _ = io::stdin()
        .read_line(&mut input_number)
        .expect("a valid entry");

    let mut input_number = match input_number.trim().parse::<i16>() {
        Ok(number_value) => number_value,
        Err(error) => {
            println!("erro: {}", error);
            0
        }
    };

    if !(input_number >= -32 && input_number <= 1000) {
        return;
    }

    loop {
        println!("insert type of conver 'C' or 'F'");
        input_type_grades.clear();
        let _ = io::stdin().read_line(&mut input_type_grades);
        if input_type_grades.trim() == "C" || input_type_grades.trim() == "F" {
            break;
        }
    }

    if input_type_grades.trim() == "C" {
        println!("Converte in Celcius");
        convert_to_celcius(&mut input_number);
    }

    if input_type_grades.trim() == "F" {
        println!("Converte in Farenheit");
        convert_to_farenheit(&mut input_number);
    }

    println!(
        "Convert grades in {} : {}",
        input_type_grades.trim(),
        input_number
    );
}

fn convert_to_farenheit(celcius: &mut i16) {
    *celcius = *celcius * (9 / 5 + 32);
}

fn convert_to_celcius(farenheit: &mut i16) {
    *farenheit = (*farenheit - 32) * 5 / 9
}
