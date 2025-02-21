//variable and mutability
//variable is immutable by default
//use mut keyword to make it mutable

fn main() {
    println!("Hello, world!");
    let mut a: u32 = 5;
    println!("The value of a is: {}", a);
    a = 6; //error: cannot assign twice to immutable variable
    println!("The value of a is: {}", a);
}
