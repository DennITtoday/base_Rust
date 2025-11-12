use std::io;

fn main() {
    let mut c = String::new();
    
    io::stdin().read_line(&mut c).expect("NOT A VALID GRADUS");
    
    let mut gradus_celsia: f32 = c.trim().parse().unwrap();
    let d: f32 = 1.8;
    let num: f32 = 32.0;
    
    println!("{}", (gradus_celsia*d)+num)
}
