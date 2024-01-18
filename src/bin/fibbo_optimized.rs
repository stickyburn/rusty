use std::io;

fn fibbo(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut temp;

    for _ in 0..n {
        temp = a;
        a = b;
        b = temp + b;
    }
    a
}

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("invalid number");

    let num: u64 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let res = fibbo(num);
    println!("Fibbo: {res}");
}
