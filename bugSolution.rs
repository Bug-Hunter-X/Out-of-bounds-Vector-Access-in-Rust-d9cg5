fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    match vec.get(2) {
        Some(value) => println!("The value is: {}", value),
        None => println!("Index out of bounds"),
    }
} 