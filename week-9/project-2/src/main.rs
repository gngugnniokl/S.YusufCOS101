use std::io::Write;

fn main() {

    // So, we got our crew of students, ya know?
    let student_names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Bianca Edemoh"];
    let matric_numbers = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let departments = vec!["Accounting", "Economics", "Computer Science", "Electrical Engineering", "Mechanical Engineering"];
    let levels = vec!["300", "100", "200", "200", "100"];

    // Opening the file where we gonna drop all the deets
    let mut file = std::fs::File::create("school.txt").expect("create failed");

    // Writing down the headers for the vibe
    file.write_all(format!("{} , {} , {} , {}\n", "Name", "Matric Number", "Department", "Level").as_bytes()).expect("write failed");

    // Looping through the squad to spill the tea on each student
    for n in 0..student_names.len() {
        file.write_all(format!("{} , {} , {} , {}\n", student_names[n], matric_numbers[n], departments[n], levels[n]).as_bytes()).expect("write failed");
    }

    // Letting the console know we're done dropping the 411
    println!("Student deets written in school.txt");
}
