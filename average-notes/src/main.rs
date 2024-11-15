#[derive(Debug)]
struct Student {
    name: String,
    calification: u8,
}

fn calculate_average(list_students: &Vec<Student>) -> f32 {
    let mut average: u32 = 0;
    for studen in list_students {
        average += studen.calification as u32;
    }

    average as f32 / list_students.len() as f32
}

fn main() {
    let studen1 = Student {
        name: String::from("Juan"),
        calification: 10,
    };
    let studen2 = Student {
        name: String::from("Pedro"),
        calification: 9,
    };
    let studen3 = Student {
        name: String::from("Maria"),
        calification: 8,
    };
    let studen4 = Student {
        name: String::from("Ana"),
        calification: 7,
    };
    let studen5 = Student {
        name: String::from("Luis"),
        calification: 6,
    };

    let list_student = vec![studen1, studen2, studen3, studen4, studen5];

    println!(
        "the average califications is {}",
        calculate_average(&list_student)
    );
}
