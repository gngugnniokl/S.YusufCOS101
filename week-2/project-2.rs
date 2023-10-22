fn main() {
    // Prices
    let t = 450000;
    let h = 750000;
    let m = 1500000;
    let d = 2850000;
    let a = 250000;

    // Quantities storage
    let t_amount = 2;
    let h_amount = 3;
    let m_amount = 1;
    let d_amount = 3;
    let a_amount = 1;

    // Amount price product storage
    let t_sum = t * t_amount;
    let h_sum = h * h_amount;
    let m_sum = m * m_amount;
    let d_sum = d * d_amount;
    let a_sum = a * a_amount;

    // Sum storage
    let summation = t_sum + h_sum + m_sum + d_sum + a_sum;

    // Average storage
    let sum_quantity = t_amount + h_amount + m_amount + d_amount + a_amount;
    let average = summation as f64 / sum_quantity as f64;

    // outtput
    println!("Summation: ${}", summation);
    println!("Average Price: ${:.2}", average);
}


