use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num1);
    io::stdin().read_line(&mut num2);
    let sum = num1 + &num2;
    println!("{sum}");
    
}
