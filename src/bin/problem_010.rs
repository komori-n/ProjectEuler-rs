use project_euler_rs::prime::Primes;

fn main() {
	let primes = Primes::new(2_000_000);
	let ans = primes.iter().sum::<u64>();
	println!("{}", ans);
}
