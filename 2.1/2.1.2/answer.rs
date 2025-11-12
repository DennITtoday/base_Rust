use std::io;

fn main () {
    let mut num1 = String::new();
    let mut num2 = String::new();

    io::stdin().read_line(&mut num1).expect("ERROR WHILE READING FIRST NUMBER");    
    io::stdin().read_line(&mut num2).expect("ERROR WHILE READING SECOND NUMBER");
    
    let num_1: i32 = num1.trim().parse().unwrap();
    let num_2: i32 = num2.trim().parse().unwrap();
    
    println!("{}", num_1+num_2);
}