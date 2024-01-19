enum Colors {
    Green,
    Red,
    Rbg { r: str, b: str, g: String },
}

impl Colors {
    fn print_colors(&self) {
        match self {
            Colors::Green => println!("Greee"),
            Colors::Red => println!("Reee"),
            Colors::Rbg { r, b, g } => println!("Colzz {} {} {}", r, g, b),
        }
    }
}

fn main() {
    let cols = Colors::Rbg { r: "red", b: "bb", g: String::from("gree") }
}
