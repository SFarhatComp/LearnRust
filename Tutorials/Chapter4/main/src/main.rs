// Example: Each value in Rust has a variable that's its owner. 

// fn main(){

//     let s1 = String::from("Rust"); // at this point only s1 holds the value "Rust"
//     let len  = calculate_length(&s1);

//     println!("The length of '{}' is {}", s1, len);
// }


// fn calculate_length(s: &str) -> usize{
//     s.len()
// }



// fn main(){

//         let s1 = String::from("Rust"); // at this point only s1 holds the value "Rust"
//         let s2 = s1; // s2 is now the owner of the value "Rust" and s1 is no longer valid    
//         println!("The length of '{}' is {}", s2, calculate_length(&s2)); // we can still use s2 but not s1 since s1 is no longer valid
//     }
    
    
//     fn calculate_length(s: &str) -> usize{
//         s.len()
//     }


fn main(){

    let s1 = String::from("Rust"); // at this point only s1 holds the value "Rust"
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len); // we can still use s2 and s1
}

fn calculate_length(s: &str) -> usize{
    s.len()
}