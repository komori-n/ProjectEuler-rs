use std::cmp::max;

use project_euler_rs::prime::Primes;

fn main() {
	let primes = Primes::new(1_000_000);

	let mut ans = 0;
	let mut max_n = 0;
	for a in -999..1000 {
		for b in primes.iter().take_while(|p| **p < 1000).map(|b| *b as i64) {
			let func = |n: i64| n * n + a * n + b;

			let cnt = (0..)
				.skip_while(|i| {
					let y = func(*i as i64);
					primes.is_prime(y as u64)
				})
				.next()
				.unwrap();

			if max_n < cnt {
				max_n = cnt;
				ans = a * b;
			}
		}
	}

	println!("{}", ans);
}
