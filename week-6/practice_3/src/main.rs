fn main() {
    let name1 = "Ayomide Adesokan";
    println!("My name is {}",name1);

    //find and replace
    let name2 = name1.replace("Ayomide","Adebare");
    println!("You can call me {}",name2);
    let facaulty = "Facaulty of Science and Technology";

    //find and replace
    let school = facaulty.replace("facaulty","school");
    println!("I am a student of the {}", school);
}
