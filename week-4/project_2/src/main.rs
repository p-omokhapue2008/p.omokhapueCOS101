use std::io;

fn main() {
    // Read the employee's experience and age
    println!("Enter employee's years of experience: ");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read line");
    let experience: u32 = experience_input.trim().parse().expect("Please enter a valid number for experience");

    println!("Enter employee's age: ");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number for age");

    // Determine the incentive based on experience and age
    let incentive = if experience > 5{
        // Experienced employee
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            1_300_000
        }
    } else {
        // Inexperienced employee
        100_000
    };

    // Print the annual incentive
    println!("The annual incentive for the employee is: N {}",incentive );
}
