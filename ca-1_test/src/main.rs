//Made by Yusuf Salvation Ipinuoluwa(SEN, COS 101)
use std::io;
fn main() {
    /*Program to ask for name,date of birth, diagnosis, village of residency,Email address
    phone number, number of siblings, Number of children
     */
    //Block for variable declaration
    let mut name:String = String::new();
    let mut date_of_birth = String::new();
    let mut number_of_siblings = String::new();
    let mut diagnosis:String = String::new();
    let mut email = String::new();
    let mut phone_number = String::new();
    let mut number_of_children = String::new();
    let mut village_of_residency:String = String::new();
    let alz_cost:i32 = 1200000;
    let arrh_cost:i32 = 550000;
    let ckd_cost:i32 = 15000000;
    let dia_cost:i32 = 800000;
    let arth_cost:i32 = 450000;
    let diag_1:&str = "Alzheimer";
    let diag_2:&str = "Chronic kidney disease";
    let diag_3:&str = "Arrhythmia";
    let diag_4:&str = "Diabetes";
    let diag_5:&str = "Arthritis";
    let vor_1:&str = "Akpabom";
    let vor_2:&str = "Atabrikang";
    let vor_3:&str = "Ngbaugi";
    let vor_4:&str = "Okorobilom";
    let vor_5:&str = "Emeremen";
    //Block where program outputs user prompts
    println!("Hello there. What is your name?");
    io::stdin().read_line(&mut name).expect("Wrong input type");
    println!("Ok {}, What is your Village of residency?", name);
    io::stdin().read_line(&mut village_of_residency).expect("Wrong input type");
    println!("What is your Email?");
    io::stdin().read_line(&mut email).expect("Wrong input type");
    println!("What is your phone number?");
    io::stdin().read_line(&mut phone_number).expect("Wrong input type");
    println!("How many children do you have??");
    io::stdin().read_line(&mut number_of_children).expect("Wrong input type");
    let number_of_children_mod:i32 = number_of_children.trim().parse().expect("Wrong input type");
    println!("How many siblings do you have?");
    io::stdin().read_line(&mut number_of_siblings).expect("Wrong input type");
    let number_of_siblings_mod:i32 = number_of_siblings.trim().parse().expect("Wrong input type");
    println!("What is your medical diagnosis?");
    io::stdin().read_line(&mut diagnosis).expect("Wrong input type");
    println!("When were you born?");
    io::stdin().read_line(&mut date_of_birth).expect("Wrong input type");
    let alz_cost_final:f64 = ((alz_cost as f64 * 0.2 as f64) - alz_cost as f64) as f64;
    let date_of_birth_mod:i32 = date_of_birth.parse().trim().expect("Invalid data type");
    let age:i32 = 2023 - date_of_birth_mod;
    //If and conditionals block
    if diagnosis == diag_1 && age > 50  &&number_of_children_mod > 4 && village_of_residency == vor_1 {
    println!("Your final cost of treatement is {}", alz_cost_final);
    }else {
        println!("Your cost of treatement is {}", alz_cost);
    }
}
