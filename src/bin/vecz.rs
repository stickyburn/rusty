fn main() {
    let nums = vec![10, 20, 30];

    for num in nums {
        match num {
            30 => println!("thirty"),
            _ => println!("{}", num),
        }
    }
}
