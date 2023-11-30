/*Your MTH 101 Professor has asked you to develop a Rust program 
that performs the following calculations:
Area of Trapezium formula = height/2*(base1+base2)
Area of the rhombus formula = ½ × diagonal1 × diagonal2
Area of Parallelogram formula = base x altitude
Area of Cube formula = 6 x (length of the side)2
Volume of Cylinder formula = π*radius2
*height
Using your knowledge of Rust Functions, develop the program that 
prompts a user to select an equation, reads inputs and then performs 
the corresponding calculations.
*/
use std::io;
fn rhomb_form(){
    let mut diagonal1 = String::new();
    let mut diagonal2 = String::new();

    println!("Enter the length of the first diagonal");
    io::stdin().read_line(&mut diagonal1).expect("Could not read input");
    let diagonal1:f64 = diagonal1.trim().parse().expect("Program panicked");
    println!("Enter the length of the second diagonal");
    io::stdin().read_line(&mut diagonal2).expect("Could not read input");
    let diagonal2:f64 = diagonal2.trim().parse().expect("Program panicked");
    
    let rhombus_area:f64 = diagonal1 * diagonal2 * 0.5;
    println!("The area of your rhombus is {}", rhombus_area);
}
fn trap_form() {
    //variable declaration for trap_form function
    let mut trapheight = String::new();
    let mut trapbase1 = String::new();
    let mut trapbase2 = String::new();
    //user prompt for trap_form function
    println!("Enter the height of the trapezium");
    io::stdin().read_line(&mut trapheight).expect("Could not read input");
    let trapheight:f64 = trapheight.trim().parse().expect("Program panicked");
    println!("Enter the base1 of the trapezium");
    io::stdin().read_line(&mut trapbase1).expect("Could not read input");
    let trapbase1:f64 = trapbase1.trim().parse().expect("Program panicked");
    println!("Enter the base2 of the trapezium");
    io::stdin().read_line(&mut trapbase2).expect("Could not read input");
    let trapbase2:f64 = trapbase2.trim().parse().expect("Program panicked");
    let trap_area = trapheight/2.0*(trapbase1+trapbase2);
    println!("The area of your trapezium is {}", trap_area);
}
fn par_form(){
//Variable declaration for the 
    let mut parbase = String::new();
    let mut paralt = String::new();
    //user prompt for trap_form function
    println!("Enter the height of the parallelogram");
    io::stdin().read_line(&mut paralt).expect("Could not read input");
    let paralt:f64 = paralt.trim().parse().expect("Program panicked");
    println!("Enter the base of the parallelogram");
    io::stdin().read_line(&mut parbase).expect("Could not read input");
    let parbase:f64 = parbase.trim().parse().expect("Program panicked");
    let par_area:f64 = parbase * paralt ;
    println!("The area of your parallelogram is {}", par_area);
}
fn cube_vol(){
    let mut side = String::new();
    //user prompt for trap_form function
    println!("Enter the length of the side of the cube");
    io::stdin().read_line(&mut side).expect("Could not read input");
    let side:f64 = side.trim().parse().expect("Program panicked");
    let cube_volu:f64 = 6.0 * side.powf(2.0) ;
    println!("The volume of your cube is {}", cube_volu);
}
fn cyli_vol(){
    let mut radius = String::new();
    let mut height = String::new();
    //user prompt for trap_form function
    println!("Enter the height of the cylinder");
    io::stdin().read_line(&mut height).expect("Could not read input");
    let height:f64 = height.trim().parse().expect("Program panicked");
    println!("Enter the radius of the cylinder");
    io::stdin().read_line(&mut radius).expect("Could not read input");
    let radius:f64 = radius.trim().parse().expect("Program panicked");
    let cyli_volu:f64 = radius.powf(2.0) * height * std::f64::consts::PI ;
    println!("The volume of your cylinder is {}", cyli_volu);
}
fn main(){
    let mut input1 = String::new();
    println!("What would you like to calculate?((1)Area of parallelogram, (2)Area of trapezium, (3) Area of a rhombus, (4)Volume of cube, (5)Volume of cylinder)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let trimmed_input = input1.trim();
    match trimmed_input {
        "1" => par_form(),
        "2" => trap_form(),
        "3" => rhomb_form(),
        "4" => cube_vol(),
        "5" => cyli_vol(),
        _ => println!("Invalid input"),
    }
}

