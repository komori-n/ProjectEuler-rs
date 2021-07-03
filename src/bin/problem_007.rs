use project_euler_rs::prime::Primes;

fn main() {
	let primes = Primes::new(1_000_000);
	println!("{}", primes.iter().nth(10_001 - 1).unwrap());
}
