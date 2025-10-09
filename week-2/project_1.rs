fn main() 
{
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	//calculate amount
	let a = p * (1.0 + (r / 100.0)).powf(n);

	//calculate compound interest
	let ci = a - p;

	//print compound interest
	println!("compound interest is {}", ci );
}