fn main() {
    let fullname = "Chibudun John Umeh";
    let department = "Computer Science";
    let uni = "Pan atlantic University";
    

    let mut school= "School of Science".to_string();
    // push stringg
    school.push_str(" and Technology");

    println!("My name is: {}", fullname);
    //check length
    println!("The length my fullname is: {}", fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}",school);
    println!("{}",uni);
}
