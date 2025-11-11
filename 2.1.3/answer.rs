use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    
    io::stdin().read_line(&mut num1).expect("NOT A VALID NUMBER");
    io::stdin().read_line(&mut num2).expect("NOT A VALID NUMBER2");
    
    let num_1: f32 = num1.trim().parse().unwrap();
    let num_2: f32 = num2.trim().parse().unwrap();
    
    println!("{}", num_1*num_2);
}
