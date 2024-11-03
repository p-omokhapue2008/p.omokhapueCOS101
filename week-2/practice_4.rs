<<<<<<< HEAD
fn main() {
	let p:f64 =1000.0;
	let r:f64 =1.0;
	let t:f64 =2.0;

	// simple interest
	let a = p * (1.0 + (r / 100.0)) * t;
	println!("Amount is {}", a);
	let si = a - p;
	println!("Simple Interest is {}", si);
	
=======
fn main() {
	let p:f64 =1000.0;
	let r:f64 =1.0;
	let t:f64 =2.0;

	// simple interest
	let a = p * (1.0 + (r / 100.0)) * t;
	println!("Amount is {}", a);
	let si = a - p;
	println!("Simple Interest is {}", si);
	
>>>>>>> 6617a2714a8b1cae44a9667836187f0db673a3e3
}