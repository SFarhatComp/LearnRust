// Compound Data Types

//arrays, tuples, slices, and strings (slice strings)



fn main(){

// Arrays
// Arrays are fixed-size list where elements are of the same data type

let list_1: [i32; 5 ]= [1, 2, 3, 4, 5]; // [i32; 5] 5 elements of i32
println!("Array list_1 {:?}", list_1);

// let mix = [1, 2, "apple", true]; // 
// println!("Array mix {:?}", mix);

let fruits: [&str; 3] = ["apple", "banana", "orange"];
println!("Array fruits {:?}", fruits);
println!("Array fruits {:?}", fruits[0]);
println!("Array fruits {:?}", fruits[1]);
println!("Array fruits {:?}", fruits[2]);

// Tuples
// Tuples are fixed-size list where elements can be of different data types
let human:(String, i8, bool) = ("John".to_string(), 25, false);
println!("Tuple human {:?}", human);

let mix_tuple = ("Michael", 23, true, [1,2,3,4,5]);
println!("Tuple mix_tuple {:?}", mix_tuple);


// Slices [1,2,3,4,5] -> adjacent elements in memory memory_adress + offset = next memory address 
// Slices are a reference to a contiguous sequence of elements in a collection
let number_slices:&[i32] = &[1,2,3,4,5]; // [2, 3]
println!("Slice number_slices {:?}", number_slices);

let animal_slices:&[&str] = &["cat", "dog", "lion"]; 
println!("Slice number_slices {:?}", animal_slices);


let book_slices:&[&String] = &[&"cat".to_string(), &"dog".to_string(), &"lion".to_string()]; 
println!("Slice number_slices {:?}", book_slices);

// Strings

// main difference between strings and string slice is that the strings are growable/expandable/mutable and they are owned not borrowd 
// memory allocation and efficiency is super important in Rust
// Best off C++ and C in terms of memory management
// Strings are allocated on the heap, meaning they can store an amount of text that is unknown to us at compile time, like using the new keyword in C++ 

// all data type by default are immutable in Rust

let mut stone_cold: String = String::from("Hell, "); // atp stone_cold is stored on heap

stone_cold.push_str("Yeah"); // push_str is a method that appends a string slice to a string

println!("String stone_cold {}", stone_cold);

// String Slice is a reference to a sequence of bytes in a string. They are immutable and they are used to reference a part of a string. No need to copy or own the data,
// just borrow it. they are created on the stack


// Stack is faster than heap, but has a limited size. It also only has imyuutable data types

//&str

let string: String = String::from("Hello, World");
let slice: &str = &string[0..5]; // [0..5] is a range that means from 0 to 5
println!("String slice {}", slice);
}

fn test(){
    println!("SLICE: {}", slice);
}