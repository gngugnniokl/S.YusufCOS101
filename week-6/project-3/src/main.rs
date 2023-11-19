use std::io;
fn main() {
    let mut input_n = String::new();
    println!("Enter an input");
    io::stdin().read_line(&mut input_n).expect("Wrong input type");
    let input_n:i32 = input_n.trim().parse().expect("Wrong input type");
        for i in 1..12 {
        let x = i * input_n;
    println!("{}", x);
    }
}
