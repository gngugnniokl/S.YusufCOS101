use std::io;
/*You overheard your Dad and Uncle arguing about “formula 1”
and “Needs for speed”. To bring an end to their argument,
they asked you to develop a rust program that tells them how
fast a car is traveling (in kilometers) if it goes 80 miles in 2
hours.
How about if it goes 120 miles in 4 hours?*/
fn main() {
    //chunk of code dedicated to getting distance
let mut unc_distance = String::new();
println!("Input the distance in miles");
io::stdin().read_line(&mut unc_distance).expect("You screwed up lad. Abeg re-enter your string");
let unc_distance:i32 = unc_distance.trim().parse().expect("Not right type of value blud. rerun program and enter again");
let conv_distance = unc_distance as f64*1.609344;
println!("Your converted distance is {} km", conv_distance);
//Chunk of code dedicated to getting time in hours. That is time in hours unconverted. Since the time unconverted inputed by the user is in hours
let mut time = String::new();
println!("Enter the amount of time the trip took in hours");
io::stdin().read_line(&mut time).expect("Wrong datatype inputed bro");
let time:f64 = time.trim().parse().expect("Wrong datatype brooooo");
println!("The time you inputed is {} hours", time);
//Chunk of code dedicated to getting speed in km/h
let speed = conv_distance/time;
println!("Your vehicular speed is {} km/h", speed);
}
