use project_euler_rs::prime::Primes;

fn main() {
	let num = 600851475143u64;
	let primes = Primes::new((num as f64).sqrt().floor() as u64 + 1);

	println!("{:?}", primes.factorize(num));
}
