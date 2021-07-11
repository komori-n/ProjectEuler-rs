use project_euler_rs::prime::Primes;

fn main() {
	let primes = Primes::new(1_000_000);

	let ans = (2..10_001)
		.filter(|i| is_amicable(&primes, *i))
		.sum::<u64>();

	println!("{}", ans);
}

fn is_amicable(primes: &Primes, n: u64) -> bool {
	let m = primes.sum_divisors(n) - n;

	m != n && primes.sum_divisors(m) - m == n
}
