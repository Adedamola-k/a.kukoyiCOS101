//Let it know you are creating a blue print for a laptop
struct Laptop {
	brand: String,     //each laptop will have a brand name
	price: i32,       //each laptop will have a price
}

impl Laptop {
	fn total_cost(&self, qty: i32) -> i32 {   //to calculate the total cost
		self.price*qty       //multiply price with the quantity
	}
}

fn main() {
	//create a laptop called HP
	let hp = Laptop {
		brand: "HP".to_string(),    //the brand name is HP
		price: 650_000,
	};

	let ibm = Laptop {
		brand: "IBM".to_string(),
		price: 755_000,
	};

	let toshiba = Laptop {
		brand: "Toshiba".to_string(),
		price: 550_000,
	};

	let dell = Laptop {
		brand: "Dell".to_string(),
		price: 850_000,
	};

	let qty = 3;

	let total_hp = hp.total_cost(qty);
	let total_ibm = ibm.total_cost(qty);
	let total_toshiba = toshiba.total_cost(qty);
	let total_dell = dell.total_cost(qty);

	let grand_total = total_hp + total_ibm + total_toshiba + total_dell;
	println!("Grand Total Cost: {}", grand_total);

}