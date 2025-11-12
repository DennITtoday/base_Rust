use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut num3 = String::new();
    
    io::stdin().read_line(&mut num1).expect("ENTER A VALID NUM 1");
    io::stdin().read_line(&mut num2).expect("ENTER A VALID NUM 2");
    io::stdin().read_line(&mut num3).expect("ENTER A VALID NUM 3");
    
    let mut num_1: f32 = num1.trim().parse().unwrap();
    let mut num_2: f32 = num2.trim().parse().unwrap();
    let mut num_3: f32 = num3.trim().parse().unwrap();
    
    println!("{}", (num_1+num_2+num_3)/3.0)
}
