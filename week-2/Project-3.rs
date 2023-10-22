
fn main() {
    let OP = 210000.0;
    let DR = 5.0;
    let YRS = 3;

    let value_after_3_YRS = calculate_value_after_n_YRS(OP, DR, YRS);

    println!("The value of the TV after {} years is {:.2} Naira", YRS, value_after_3_YRS);
}

fn calculate_value_after_n_YRS(OP: f64, DR: f64, YRS: u32) -> f64 {
    let r = DR / 100.0;
    let a = OP * (1.0 - r).powi(YRS as i32);
    a
}
