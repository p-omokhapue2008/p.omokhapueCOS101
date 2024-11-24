use std::io;

fn main() {
    
let count in  0..=100{};
for mut count in 0:=100;

loop {
let mut input = String::new();

    println!("\nEnter your name:");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let name:String = input.trim().parse().expect("Not a valid number");

    let mut input = String::new();

    println!("\nHow many papers have you published?");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let no_of_papers:i64 = input.trim().parse().expect("Not a valid number");

    if no_of_papers >=3 && no_of_papers <=5 {
    println!(" Your incentive is N500 000");
    };

    if no_of_papers >5 && no_of_papers <10 {
    println!("Your incentive is N800 000");
    };

    if no_of_papers >=10 {
    println!("Your incentive is N1 000 000");
    };

    if no_of_papers >3 {
    println!(" Your incentive is N100 000");
    };
}

