// Constants and Mutability

fn main() {
    println!("Hello, world!");

    let mut x = 5;
    const Y: u32 = 100_000;
    println!("The value of x is: {}", x);
    println!("The value of Y is: {}", Y);

    println!("The value of PI is: {}", PI);
}

const PI: f64 = 3.14159;
// You can declare a constant in any scope, including the global scope, which makes it a useful tool for values that many parts of your code need to know about.
// Constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
// Constants are valid for the entire time a program runs, within the scope they were declared in, making them a useful choice for values your code needs to know about throughout the program.
// The naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability.
// Constants are declared using the const keyword instead of the let keyword, and the type of the value must be annotated.



