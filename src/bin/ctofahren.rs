use std::io;

fn c_to_f(input: f32) -> f32 {
    (input * 9.0 / 5.0) + 5.0 / 9.0
}

fn f_to_c(input: f32) -> f32 {
    (input - 32.0) * 5.0 / 9.0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading");
    let unit = input
        .trim()
        .chars()
        .next()
        .unwrap()
        .to_uppercase()
        .to_string();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let temperature: f32 = input.trim().parse().expect("Please enter a valid number");

    match unit.as_str() {
        "C" => {
            let converted = c_to_f(temperature);
            println!("{:.2}°C is {:.2}°F", temperature, converted);
        }
        "F" => {
            let converted = f_to_c(temperature);
            println!("{:.2}°F is {:.2}°C", temperature, converted);
        }
        _ => {
            println!("Hmmm");
        }
    }
}
