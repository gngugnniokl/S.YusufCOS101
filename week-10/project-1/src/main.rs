struct Laptops {
    hp: u32,
    ibm: u32,
    toshiba: u32,
    dell: u32,
}

impl Laptops {
    fn sum(&self) -> u32 {
        3 * (self.hp + self.ibm + self.toshiba + self.dell)
    }
}

fn main() {
    let small = Laptops {
        hp: 650_000,
        ibm: 755_000,
        toshiba: 550_000,
        dell: 850_000,
    };

    println!("The total cost from buying 3 from each brand is {}", small.sum());
}
