use project_euler_rs::prime::Primes;
use project_euler_rs::triangular::Triangular;

fn main() {
	let primes = Primes::new(1_000_000);
	let ans = Triangular::new()
		.skip_while(|x| primes.factor_num(*x) < 500)
		.next()
		.unwrap();

	println!("{}", ans);
}
