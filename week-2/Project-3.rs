fn main() {
    let initial_value = 210000.0;
    let annual_depreciation_rate = 0.05; // 5% annual depreciation rate
    let years = 3;

    // Calculate the value of the TV after 3 years using the formula
    let final_value = initial_value * (1.0 - annual_depreciation_rate).powf(years as f64);

    println!("The value of the TV after {} years is: N{:.2}", years, final_value);
}









