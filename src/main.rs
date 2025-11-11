use std::io;

fn main() {
    let mut num = String::new();
    
    io::stdin().read_line(&mut num).expect("ENTER A VALID NUM");
    
    let input: f64 = num.trim().parse().unwrap();
    
    println!("{}", ((input%100.0)%10.0)+((input-(input%100.0))/100.0)+(((input%100.0))-((input%100.0)%10.0))/10.0)
}
