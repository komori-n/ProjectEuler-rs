use num::pow;
use project_euler_rs::prime::Primes;

fn main() {
	let primes = Primes::new(1_000_000);

	let ans = primes
		.iter()
		.filter(|p| circular_numbers(**p).iter().all(|q| primes.is_prime(*q)))
		.collect::<Vec<_>>();

	println!("{}", ans.len());
}

fn circular_numbers(n: u64) -> Vec<u64> {
	let mut m = n;
	let digit_num = 1 + (n as f64).log10().floor() as usize;
	let mut ans = vec![n];

	loop {
		m = (m / 10) + ((m % 10) * pow(10, digit_num - 1));
		if n == m {
			return ans;
		}
		ans.push(m);
	}
}
