use std::io;

fn main() {
   let mut developers: Vec<(String, u32)> = Vec::new();

   //Ask how many people you want to enter
     println!("How many developers are being interviewed?");
    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).expect("Failed to read input");
    let count: usize = count_input.trim().parse().expect("Invalid number");

   for _ in 0..count {
   		 //Ask for name
   		 let mut name_input = String::new();
		println!("What is your name?");
		io::stdin().read_line(&mut name_input).expect("failed to read input");
		let name = name_input.trim().to_string();

		//Ask for experience
		let mut experience_input = String::new();
		println!("Enter your years of experience" );
		io::stdin().read_line(&mut experience_input).expect("failed to read input");
   	 	let experience: u32 = experience_input.trim().parse().expect("Invalid number");

    	//Store in vector
    	developers.push((name,experience));
    }

	// Assume the first person is the most experienced
    let mut most_experienced = &developers[0];

    // Compare
    for dev in &developers {
        if dev.1 > most_experienced.1 {
            most_experienced = dev;
        }
    }

    println!(
        "\nThe most experienced developer is {} with {} years.",
        most_experienced.0, most_experienced.1
    );
}