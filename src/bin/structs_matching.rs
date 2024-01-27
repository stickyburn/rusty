struct Person {
    name: String,
    age: u8,
}

fn main() {
    let people = vec![
        Person {
            name: String::from("User1"),
            age: 12,
        },
        Person {
            name: String::from("User2"),
            age: 17,
        },
        Person {
            name: String::from("User3"),
            age: 21,
        },
    ];

    for person in people {
        match person.age {
            0..=12 => println!("Boo kid"),
            13..=17 => println!("Almost {}", person.name),
            _ => println!("Yeet! {}", person.name),
        }
    }
}
