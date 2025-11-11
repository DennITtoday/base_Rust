use std::io;

fn main() 
{
    let mut input = String::new();

    io::stdin().read_line(&mut input)
    .expect("ERROR WHILE READING STRING INPUT");

    let num: i32 = input.trim().parse().unwrap();

    println!("{}", num*num);

}
