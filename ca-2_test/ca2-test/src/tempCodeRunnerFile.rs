//importing function crates
use std::io;
use std::fs::File;
use std::io::prelude::*;
//struct and method declaration
struct Company {
    shares:f32,
    liabilities:f32,
    founding_date:i32,
    username:String,
    password:String
}
impl Company{
    fn percentage_leverages(&self)-> f32{
        return self.liabilities / self.shares * 100.0;
    }
    fn percentage_leverages_cond(&self)-> f32{
         return self.percentage_leverages() * 0.005 ;
    }
}  
fn main() {
    let champion = Company {
        shares:250000000.0,
        liabilities:8000000.0,
        founding_date:1974,
        username:String::from("cham"),
        password:String::from("cham123")
    };
    let dangote_sugar = Company {
        shares:18000000.0,
        liabilities:10000000.0,
        founding_date:1970,
        username:String::from("dang"),
        password:String::from("dang123")
    };
    let flour_mills = Company {
        shares:32000000.0,
        liabilities:4000000.0,
        founding_date:1960,
        username:String::from("flou"),
        password:String::from("flou123")
    };
    let nestle = Company {
        shares:8000000.0,
        liabilities:1500000.0,
        founding_date:1961,
        username:String::from("nest"),
        password:String::from("nest123")
    };
    let unilever = Company {
        shares:37000000.0,
        liabilities:11000000.0,
        founding_date:1923,
        username:String::from("unil"),
        password:String::from("unil123")
    };
    let honeywell = Company {
        shares:34000000.0,
        liabilities:9000000.0,
        founding_date:1906,
        username:String::from("hone"),
        password:String::from("hone123")
    };
    let nigerian_breweries_plc = Company {
        shares:30000000.0,
        liabilities:12000000.0,
        founding_date:1906,
        username:String::from("nige"),
        password:String::from("nige123")
    };
    //variable declaration
    let mut username = String::new();
    let mut password = String::new();
    let mut found_company = None;
    let companies = vec![champion, dangote_sugar, flour_mills, nestle, unilever, honeywell, nigerian_breweries_plc];
    let mut logged_in_company_leverage = 0.0;
    //loop activation
loop {
    //prompt for username
println!("Good day. What is your username?");
io::stdin().read_line(&mut username).expect("Invalid input");
let updated_username = username.trim();
//conditionals check for username
if updated_username.chars().count() <3 || updated_username.chars().count() > 8{
println!("Invalid username please try again");
continue;
} else{
    username = updated_username.to_string(); 
    break;}
}
//loop activation
loop{
    //prompt for password
println!("Welcome {}, Now enter a password: ", username);
io::stdin().read_line(&mut password).expect("Invalid data type");
let updated_password = password.trim();
//password conditionals
if !updated_password.chars().all(|c| c.is_ascii_lowercase() || c.is_digit(10))  {
    println!("Invalid characters inputted. Please enter a password again");
    continue;
}
 else {
    password = updated_password.to_string(); 
    break;
}
} 
for company in companies.iter(){
    if company.password == password && company.username == username {
        found_company = Some(company);
        break;
    }
}
//conditionals check to create text file. 
//prime loop. The main loop sha. Nested loop initiation
if let Some(company) = found_company {
    let mut file = File::create("Company_info.txt").expect("File creation failed");
    if company.shares > 20000000.0 && company.liabilities > 10000000.0 && company.liabilities == 10000000{
        let company_info = format!( "Company: {:?}\nShares: {}\nLiabilities: {}\nFounding Date: {} \nleverage: {}\n\n", company.username, company.shares, company.liabilities, company.founding_date, company.percentage_leverages() );
        file.write_all(company_info.as_bytes()).expect("Write failed");
    } else if company.liabilities < 10000000.0 && company.shares == 20000000.0 || company.shares < 20000000.0 {
        let company_info = format!( "Company: {:?}\nShares: {}\nLiabilities: {}\nFounding Date: {} \nfrac_leverage: {}\n\n", company.username, company.shares, company.liabilities, company.founding_date, company.percent_leverages_cond() );
        file.write_all(company_info.as_bytes()).expect("Write failed");
    } 
    else if company.liabilities < 10000000.0 || company.shares > 20000000.0 {
            let company_info = format!( "Company: {:?}\nShares: {}\nLiabilities: {}\nFounding Date: {} \nfrac_leverage: {} \nleverage: {}\n\n", company.username, company.shares, company.liabilities, company.founding_date, company.percent_leverages_cond(), company.percentage_leverages() );
            file.write_all(company_info.as_bytes()).expect("Write failed");
    }
    else {
         println!("invalid credentials");
    }
    
}
}