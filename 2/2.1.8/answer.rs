use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("ENTER A VALID NUM");
    
    let num: f32 = input.trim().parse().unwrap();
    
    println!("{}", (num/10.0)-((num%10.0)/10.0))
}
