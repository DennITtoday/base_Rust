use std::io;

fn main() {
    let mut time = String::new();
    
    io::stdin().read_line(&mut time).expect("ENTER A VALID TIME");
    
    let t: i32 = time.trim().parse().unwrap();
    
    match t {
        6..=10 => println!("Утро"),
        11..=16 => println!("День"),
        17..=21 => println!("Вечер"),
        22..=23 | 0..=5  => println!("Ночь"),
        _ => println!("UNCORRECT"),
    }
}
