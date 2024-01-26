use std::io::Write;

fn main() {
    // Hey babes, let's spill the tea on our EFCC commissioners, shall we?
    let name_of_commisioner = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "Justice", "Defence", "Power & Steel", "Petroleum"];
    let geopolitical_zone = vec!["South West", "North East", "South South", "South West", "South East"];

    // Grab your sparkles, let's create the deets file
    let mut file = std::fs::File::create("EFCC.txt").expect("create failed");

    // Throwing in the headers, like, name, ministry, and geopolitical zone
    file.write_all(format!("{} , {} , {}\n", "Name", "Ministry", "Geopolitical Zone").as_bytes()).expect("write failed");

    // Time to spill the tea on each commissioner, y'all
    for n in 0..name_of_commisioner.len() {
        file.write_all(format!("{} , {} , {}\n", name_of_commisioner[n], ministry[n], geopolitical_zone[n]).as_bytes()).expect("write failed");
    }

    // Just FYI, we're done spilling the EFCC deets in EFCC.txt, xoxo
    println!("EFCC deets are in EFCC.txt, stay fab!");
}

