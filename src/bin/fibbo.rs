use std::io;

fn fibbo(n: u32) -> u32 {
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
    io::stdin().read_line(&mut num).expect("error reading");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let result = fibbo(num);
    println!("Fib: {result}");
}
