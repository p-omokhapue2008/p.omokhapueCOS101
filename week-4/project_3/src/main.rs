use std::io;

fn main() {
    // Define the food menu and prices
    let menu = [
        ("Poundo Yam/Edinkaiko Soup", 3200),
        ("Fried Rice & Chicken", 3000),
        ("Amala & Ewedu Soup", 2500),
        ("Eba and Egusi Soup", 2000),
        ("White Rice & Stew", 2500),
        
    ];

    println!("Welcome to the Restaurant!");
    println!("Here is the menu:");

    // Display the menu
    for (index, (food, price)) in menu.iter().enumerate() {
        println!("{} , {} - N {}", index + 1, food, *price);
    }

    let mut total_cost = 0;

    loop {
        // Prompt user for food choice
        println!("\nEnter the number of the food you want to order (1-{}), or 0 to finish:", menu.len());
        let mut choice_input = String::new();
        io::stdin().read_line(&mut choice_input).expect("Failed to read line");
        let choice: usize = choice_input.trim().parse().expect("Please enter a valid number");

        if choice == 0 {
            break;
        }

        if choice < 1 || choice > menu.len() {
            println!("Invalid choice, please select a valid option.");
            continue;
        }

        // Get the quantity for the selected food
        println!("Enter the quantity of {} you want to order:", menu[choice - 1].0);
        let mut quantity_input = String::new();
        io::stdin().read_line(&mut quantity_input).expect("Failed to read line");
        let quantity: u32 = quantity_input.trim().parse().expect("Please enter a valid number");

        // Calculate the cost for this item and add to total
        let (food, price) = menu[choice - 1];
        let cost = price * quantity;
        total_cost += cost;
        println!("You ordered {} x {} for a total of N {}", quantity, food, cost as f64);
    }

    // Apply discount if applicable
    if total_cost > 10_000 {
        let discount = total_cost as f64 * 0.05;
        let discounted_total = total_cost as f64 - discount;
        println!("\nTotal order cost: N {}", total_cost as f64);
        println!("A discount of 5% has been applied: -N {}", discount);
        println!("Total cost after discount: N {}", discounted_total);
    } else {
        println!("\nTotal order cost: N {}", total_cost as f64);
    }
}
