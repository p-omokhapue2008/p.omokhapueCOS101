fn main() {
	let t:f64 = 2.0 * 450000.0;
	let m:f64 = 1500000.0;
	let h:f64 = 3.0 * 750000.0;
	let d:f64 = 3.0 * 2850000.0;
	let a:f64 = 250000.0;

	// sum and average
	let s = t + m + h + d + a;
	println!("Sum is {}", s );
	let p = s / 10.0;
	println!("Average is {}",p );
}