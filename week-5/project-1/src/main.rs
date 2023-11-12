use std::io;
fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();
    println!("Enter your value for the coefficient to the variable raised to the second power/ your a");
    io::stdin().read_line(&mut input_1).expect("Invalid input");
    let a:f64 = input_1.trim().parse().expect("Felt cute today");
    println!("Enter your value for the coefficient to the variable raised to the first power / your b");
    io::stdin().read_line(&mut input_2).expect("Invalid input");
    let b:f64 = input_2.trim().parse().expect("Felt cute today");
    println!("Enter your value for the coefficient to the variable raised to the null power / your c");
    io::stdin().read_line(&mut input_3).expect("Invalid input");
    let c:f64 = input_3.trim().parse().expect("Felt cute today");
    let x:f64 = (b.powf(2.0) - (4.0 * a * c ));
    println!("Discriminant is{}", x );
}

