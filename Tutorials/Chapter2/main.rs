//  Primitive data types

// int, float, bool, char

// Integer
// Rust has signed and unsigned integers types of different sizes
// Signed: i8, i16, i32, i64, i128, isize ( + and - numbers)
// Unsigned: u8, u16, u32, u64, u128, usize ( only + values )


fn main() {

// Signed
// let a: i8 = 10;  i = integer, 8 = 8 bits
// let b: i16 = 20; i = integer, 16 = 16 bits
// let d: i64 = 40;
// let c: i32 = 30;  
// let e: i128 = 50;
// let f: isize = 60;
let x: i32 = - 10;

// Unsigned
// let g: u8 = 70;
// let h: u16 = 80;
// let i: u32 = 90;
// let j: u64 = 100;
// let k: u128 = 110;
// let l: usize = 120;
let y :u32 = 10;


// range : i32 (-2^31) to (2^31 - 1) same logic apply to the rest of the sizes
// range : u32 (0) to (2^32 - 1)

println!("Signed integer {}", x);
println!("Unsigned integer {}", y);
println!("Unsigned and Signed integer {} , {}", x, y);


// float [Floating point Types]
// Rust has two floating point types f32 and f64
// f32: single precision
// f64: double precision

    let pi :f64 = 3.14159265359;

    println!("Value of pi {} ", pi);

// Boolean
// Rust has a bool type for boolean values
let is_it_true: bool = true;
println!("Is it true? {}", is_it_true);

// char
// Rust has a char type for a single character
let letter: char = 'a';
println!("Letter is {}", letter);

}