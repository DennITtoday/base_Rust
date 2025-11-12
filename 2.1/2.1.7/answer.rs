use std::io;

fn read_num() -> f32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("ENTER A VALID NUMBER");
    num.trim().parse().expect("ENTER A VALID NUM")
}

fn main() {
    println!("{}",read_num()%read_num());
}
