use std::io;
fn main() {
  let mut applicant_count = 0;
  while applicant_count < 150 {
    //preemptive variable declaration
  let mut name = String::new();
  let mut  department = String::new();
  let mut state_of_origin = String::new();
  let mut email = String::new();
  let mut class_rep_ans = String::new();
  let mut cgpa_ans = String::new();
  let mut fresher_check_ans = String::new();
  //program prompts for user input
  println!("What is your name");
  io::stdin().read_line(&mut name).expect("invalid input.");
  println!("What is your department?");
  io::stdin().read_line(&mut department).expect("invalid input");
  println!("Which state are you from?");
  io::stdin().read_line(&mut state_of_origin).expect("invalid input");
  println!("What is your email?");
  io::stdin().read_line(&mut email).expect("invalid input");
  //conditionals check
  println!("Are you class representative(yes/no)");
  io::stdin().read_line(&mut class_rep_ans).expect("invalid input. Please answer yes/no");
  let class_rep_ans:bool = class_rep_ans.to_lowercase().trim() == "yes";
  if !class_rep_ans {
    println!("You are ineligible for this program");
    return;
  }
  println!("Are you above hundred level?(yes/no)");
  io::stdin().read_line(&mut fresher_check_ans).expect("invalid input. Please answer yes/no");
  let fresher_check_ans:bool = fresher_check_ans.to_lowercase().trim() == "yes";
  if !fresher_check_ans {
    println!("You are ineligible for this program");
    return;
  }
  println!("Is your cgpa above a 4.0?(yes/no)");
  io::stdin().read_line(&mut cgpa_ans).expect("invalid input. Please answer yes/no");
  let cgpa_ans:bool = cgpa_ans.to_lowercase().trim() == "yes";
  if !cgpa_ans {
    println!("You are ineligible for this program");
    return;
  }  
    println!("Your name is {}", name);
    println!("Your state of origin is {}", state_of_origin);
    println!("Your department is {}", department);
    println!("Your email is {}", email);
    applicant_count += 1;
    if applicant_count == 150 {
      break;
    }
  }  
}
