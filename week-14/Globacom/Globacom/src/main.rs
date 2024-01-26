// Comment for future program addendums. Program has been sorted into blocks for easy and convenient modification and organization :
// Crate importation
use std::io;
use std::io::Read;
//administrator privileges function block
fn admin_perms(){
   let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
//project privileges function block
fn projman_perms(){
   let mut file = std::fs::File::open("Project_tb.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
//employee privileges function block
fn emp_perms(){
   let mut file = std::fs::File::open("Staff_tb.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
//costumer privileges function block
fn cost_perms(){
   let mut file = std::fs::File::open("Customer_tb.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
//vendor privileges function block
fn vend_perms(){
   let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
fn main() {
   let mut role_input = String::new();
   println!("Hello there. Plese select your role\n");
   println!("1 if administrator\n 2 if project manager\n 3 if employee\n 4 if customer \n 5 if vendor");
   io::stdin().read_line(&mut role_input).expect("Error");
   let role_input = role_input.trim();
   //match function declaration
   match role_input {
    "1" => admin_perms(),
    "2" => projman_perms(),
    "3" => emp_perms(),
    "4" => cost_perms(),
    "5" => vend_perms(),
    _ => println!("You are ineligible to use this application")
   }
}
