fn main()
{
	let t:f64 = 450000.0 * 2.0;
	let m:f64 = 1500000.0;
	let h:f64 = 750000.0 * 3.0;
	let d:f64 = 2850000.0 * 3.0;
	let a:f64 = 250000.0;

	//calculate sum of items
	let s = t + m + h + d + a;

	//calculate average of the sales record
	let a = s / (2.0 + 1.0 + 3.0 + 3.0 + 1.0);

	//print sum 
	println!(" sum of items is {} ", s );

	//print average
	println!("average of the sales record is {}", a );
}