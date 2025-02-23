use std::collections::HashMap;

fn main() {
    // If statement example
    let number = 10;
    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }

    // While loop example 
    let mut count = 0;
    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }

    // For loop example
    let numbers = [1, 2, 3, 4, 5];
    for num in numbers.iter() {
        println!("The number is: {}", num);
    }

    // Using iter_mut() to modify the data
    let mut numbers = [1, 2, 3, 4, 5];
    for num in numbers.iter_mut() {
    *num *= 2;
    }
    println!("Modified numbers: {:?}", numbers);

    // Using into_iter() to take ownership of the data
    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers.into_iter() {
    println!("The number is: {}", num);
    
    }

// Using a tuple
let tuple = (1, "hello", 3.5);
println!("Tuple contains: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

// Using a struct
struct Person {
    name: String,
    age: u8,
}

let person = Person {
    name: String::from("Alice"),
    age: 30,
};
println!("Person's name is: {} and age is: {}", person.name, person.age);

// Using an enum
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let direction = Direction::Up;
match direction {
    Direction::Up => println!("Going up!"),
    Direction::Down => println!("Going down!"),
    Direction::Left => println!("Going left!"),
    Direction::Right => println!("Going right!"),
}

// Using a vector
let mut vec = vec![1, 2, 3];
vec.push(4);
println!("Vector contains: {:?}", vec);

// Using a hashmap
let mut scores = HashMap::new();
scores.insert(String::from("Alice"), 10);
scores.insert(String::from("Bob"), 20);
println!("Scores: {:?}", scores);

}
