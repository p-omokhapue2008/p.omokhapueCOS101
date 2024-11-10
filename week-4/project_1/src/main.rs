use std::io;

fn main() {


    println!("Enter value for a: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value for b:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value for c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not  a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");


    // Calculate the discriminant (d = b^2 - 4ac)
    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant > 0.0 {
        // Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("The roots are real and distinct: ");
        println!("Root 1: {}", root1);
        println!("Root 2: {}", root2);
    } else if discriminant == 0.0 {
        // One real root (repeated)
        let root = -b / (2.0 * a);
        println!("The root is real and repeated: {}", root);
    } else {
        // Complex roots
        let real_part = -b / (2.0 * a);
        let imaginary_part = discriminant.abs().sqrt() / (2.0 * a);

        println!("The roots are complex");
        println!("Root 1: {} + {}i", real_part, imaginary_part);
        println!("Root 2: {} + {}i", real_part, imaginary_part);
    }
}
