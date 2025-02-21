// Function: main 
// Description: The main function is the entry point of the program.
//              The println! macro is used to print the string "Hello, World!" to the console.
//              The ! indicates that println! is a macro, not a function.

//function and variables should be written in snake case
//snake case is all lowercase with underscores between words hello_world
fn main(){

    //calling the function hello_world
    hello_world();
    heights(182);
    human_id("Joe", 55, 190.2);

}


fn hello_world(){
    println!("Hello, World!");
}


fn heights(height: u32){
    println!("The height is: {}", height);
}



fn human_id(name: &str, age: u32, height: f32){
    println!("The name is: {}", name);
    println!("The height is: {}", height);
    println!("The age is: {}", age);
}


// Expression -> returns a value
// Statement -> does not return a value 