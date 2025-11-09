use std::io;

fn main() 
{
	println!("menu");

	//Print menu
	let p:f32 = 3200.0;
	println!("Price for Poundo yam and edinkaiko soup. 		CODE: P 	PRICE: {}", p); 

	let f:f32 = 3000.0;
	println!("Price for fried rice and chicken. 	CODE:F 	PRICE: {}",f);

	let a:f32 = 2500.0;
	println!("Price for amala and ewedu soup. 	CODE:A 	PRICE: {}",a);

	let e:f32 = 2000.0;
	println!("Price for eba and egusi soup. 	CODE:E 	PRICE: {}",e);

	let w:f32 = 2500.0;
	println!("Price for white rice and stew. 	CODE:W 	PRICE: {}",w);

	//Ask for food code
	let mut input1 = String::new();
	println!("Enter your food code: ");
	io::stdin().read_line(&mut input1).expect("not a valid string");
	let food_code = input1.trim();

	//Ask for quantity
	let mut input2 = String::new();
	println!("Enter the quantity: ");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let quantity:f32 = input2.trim().parse().expect("Not a valid nunber");

	//calculate the price before discount

	let price:f32;

	if food_code == "P" || food_code == "p" {
		price = p * quantity;
	}
	else if food_code == "F" || food_code == "f" {
		price = f * quantity;
	}
	else if food_code == "A" || food_code == "a" {
		price = a * quantity;
	}
	else if food_code == "E" || food_code == "e" {
		price = e * quantity;
	}
	else if food_code == "W" || food_code == "w" {
		price = w * quantity;
	}
	else {
		println!("Please enter your food code");
		price = 0.0;
	}

	//calculate the price after discount
	let price_after_discount:f32;

	if price > 10000.0 {
		price_after_discount = price - (price * 0.05);
	}

	else {
		price_after_discount = price;
	}

	println!("food code: {}", food_code);
	println!("your total price is {}", price);

	if price > 10000.0 {
		println!("price after discount {}", price_after_discount);
	}
	else {
		println!("no discount");
	}

	println!("Thanks for purchasing");
}

