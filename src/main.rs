use rand::Rng;
use std::io;

fn main() {
    let mut input1 = String::new(); // create string for the 2 input
    let mut input2 = String::new();
    println!("Welcome to the Number Generator!");

    println!("What is the first number ?"); // Prompt the user for 1st inpu
    io::stdin() // Access standard input
        .read_line(&mut input1)
        .expect("Failed to read input");

    println!("What is the second number ?"); // Prompt the user for 2nd input
    io::stdin() // okay
        .read_line(&mut input2)
        .expect("Failed to read input");

    // convert the string in numbers
    let mut num1: i32 = input1.trim().parse().expect("please enter a number");
    let mut num2: i32 = input2.trim().parse().expect("please enter a number");

    // correct the order if the first one is the higher number
    if num1 > num2 {
        std::mem::swap(&mut num1, &mut num2);
    }

    // use thoses numbers to create a random one between them
    let mut rng = rand::rng();
    let num = rng.random_range(num1..=num2);
    println!("The Random Number is : {}", num);
}
