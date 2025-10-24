//rust program to find roots of a quadratic equation

use std::io;

fn main() 
{
		println!("quadratic equation calculator");
		let mut input1 = String::new();
		let mut input2 = String::new();
		let mut input3 = String::new();

		println!("what is the value of a");
		io::stdin().read_line(&mut input1).expect("not a valid String");
		let a:f32 = input1.trim().parse().expect("not a valid number");

		println!("what is the value of b");
		io::stdin().read_line(&mut input2).expect("not a valid string");
		let b:f32 = input2.trim().parse().expect("not a valid number");

		println!("what is the value of c");
		io::stdin().read_line(&mut input3).expect("not a valid string");
		let c:f32 = input3.trim().parse().expect("not a valid number");

		//discriminant
		let d:f32 = b.powf(2.0) - 4.0*(a*c);
		let rod:f32 = d.sqrt(); //root of discriminant
		let root1:f32 = (-b - rod)/2.0*a; //first root
		let root2:f32 = (-b + rod)/2.0*a; //second root

		if d > 0.0{
			println!("there are two distinct roots: {} and {}",root1,root2);
		}

		else if d == 0.0{
			println!("there is exactly one root: {}",root1);
		}

		else{
			println!("there are no real roots");
		}
}

