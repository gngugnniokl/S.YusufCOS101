/*If the age of 
any sibling is greater than 18, ask if the sibling is married or 
single. If single then ask if a student or a worker. If a student, ask 
university and course of study. If the sibling is married, find out if 
any offspring and what city the family lives in. If the age of the 
sibling is less than 18, then find out WAEC status. If yes, then 
input secondary school attended, else input current class level. 
Finally displays the data of all siblings.
Hints:
Use a For loop to iterate the number of siblings.
Use an array to store details*/
use std::io;
fn main() {
    //Vatriable declaration
    let mut age = String::new();
    let mut marital_status = String::new();
    let mut employement_status = String::new();
    let mut university = String::new();
    let mut course_of_study = String::new();
    let mut waec_status = String::new();
    let mut class_level = String::new();
    //program prompts
    println!("How old are you?");
    io::stdin().read_line(&mut age).expect("Invalid input type");
    let age:i32 = age.trim().parse().expect("Invalid input type");
    //age conditionals
   if age > 18 {
    println!("Are you married or single?");
    io::stdin().read_line(&mut marital
   } else{
    println!("You are ineligible for this program");
    return;
   }
}
