use std::io;

// Function to get string input from user
fn get_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Function to get number input from user
fn get_number(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

fn main() {
    // Collect user information
    let name = get_string("Enter your name: ");
    let position = get_string("Enter your current position: ");
    let experience = get_number("Enter your years of experience: ");

    // Define vectors for each level
    let aps_1_2 = vec!["Intern", "-", "Paralegal", "Placement"];
    let aps_3_5 = vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"];
    let aps_5_8 = vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"];
    let el1_8_10 = vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"];
    let el2_10_13 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"];
    let ses = vec!["CEO", "Dean", "Partner", "Principal"];

    // Use match with true to determine the level
    match true {
        _ if aps_1_2.contains(&position.as_str()) && experience <= 3.0 => {
            println!("Hello {}, your APS level is: APS 1–2", name);
        }
        _ if aps_3_5.contains(&position.as_str()) && experience >= 3.0 && experience <= 5.0 => {
            println!("Hello {}, your APS level is: APS 3–5", name);
        }
        _ if aps_5_8.contains(&position.as_str()) && experience >= 5.0 && experience <= 8.0 => {
            println!("Hello {}, your APS level is: APS 5–8", name);
        }
        _ if el1_8_10.contains(&position.as_str()) && experience >= 8.0 && experience <= 10.0 => {
            println!("Hello {}, your APS level is: EL1 (8–10)", name);
        }
        _ if el2_10_13.contains(&position.as_str()) && experience >= 10.0 && experience <= 13.0 => {
            println!("Hello {}, your APS level is: EL2 (10–13)", name);
        }
        _ if ses.contains(&position.as_str()) && experience >= 13.0 => {
            println!("Hello {}, your APS level is: SES", name);
        }
        _ => {
            println!("Sorry {}, your role or experience does not match any APS level.", name);
        }
    }
}