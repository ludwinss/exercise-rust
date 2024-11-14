use rand::prelude::*;
use std::{cmp::Ordering, io};

// porque debo de usar  _ para guardar el lago de la lectura
// que significa ?
// cannot borrow rng as mutabel as it is not declared as mutabke cannot borrow as mutable
//

fn main() {
    const EXIT: &str = "exit";
    let mut rng = rand::thread_rng();

    let random_number: u8 = rng.gen_range(1..=100);
    println!("press 'exit' to close the guess game ");

    loop {
        let mut input_number = String::new();

        println!("Please. insert a number:");
        let _ = io::stdin()
            .read_line(&mut input_number)
            .expect("expect a input");

        if input_number.trim() == EXIT {
            break;
        }

        let input_number = match input_number.trim().parse::<u8>() {
            Ok(input) => input,
            Err(error) => {
                println!("error: {}", error);
                0
            }
        };

        match input_number.cmp(&random_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("u guess well");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}
