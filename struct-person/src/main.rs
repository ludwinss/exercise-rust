#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    ocuppation: String,
}

fn main() {
    let new_person = Person {
        name: String::from("Ludwinss"),
        age: 29,
        ocuppation: String::from("programmer"),
    };

    show_person_datas(&new_person);
}

fn show_person_datas(person: &Person) {
    print!("{:?}", person);
}
