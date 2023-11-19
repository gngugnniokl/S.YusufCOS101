use std::io;
fn main() {
    //loop declaration
    let mut count = 0;
    while count < 500{
        //variable declaration
        let mut number_of_papers = String::new();
        let mut name = String::new();
        //prompt for user for input and store variables
        println!("Enter your name");
        io::stdin().read_line(&mut name).expect("Invalid input");
        println!("Good day {}. Please enter your number of papers published", name);
        io::stdin().read_line(&mut number_of_papers).expect("Invalid input");
        let number_of_papers:i32 = number_of_papers.trim().parse().expect("Something went wrong");
        //condtionals check
        if number_of_papers >= 3 && number_of_papers <=5 {
            println!("Congratulations {}, your incentive is N500000", name);
        } else if number_of_papers >= 5 && number_of_papers <10 {
            println!("Congratulations {}, your incentive is N800000", name);
        } else if number_of_papers > 10 {
            println!("Congratulations {}, your incentive is N1000000", name);
        } else {
            println!("Congratulations {}, your incentive is N100000", name);
        }
        count += 1;
    }
}
