use num::pow;
use project_euler_rs::prime::Primes;

fn main() {
	let primes = Primes::new(1_000_000);
	let mut ans = vec![];
	let mut left_prev_list = vec![2, 3, 5, 7];
	let mut right_prev_list = vec![2, 3, 5, 7];

	while !left_prev_list.is_empty() && !right_prev_list.is_empty() {
		let mut left_new_list = vec![];
		let mut right_new_list = vec![];
		for &p in &left_prev_list {
			for cand in (1..10).map(|i| add_top_digit(p, i)) {
				if primes.is_prime(cand) {
					left_new_list.push(cand);
					if right_prev_list.contains(&(cand / 10)) {
						ans.push(cand);
					}
				}
			}
		}
		for &p in &right_prev_list {
			for cand in (1..10).map(|i| p * 10 + i) {
				if primes.is_prime(cand) {
					right_new_list.push(cand);
				}
			}
		}
		left_prev_list = left_new_list;
		right_prev_list = right_new_list;
	}

	println!("{}", ans.iter().sum::<u64>());
}

fn remove_top_digit(n: u64) -> u64 {
	let digit_num = 1 + (n as f64).log10().floor() as usize;
	n % pow(10, digit_num - 1)
}

fn add_top_digit(n: u64, x: u64) -> u64 {
	let digit_num = 1 + (n as f64).log10().floor() as usize;
	n + x * pow(10, digit_num)
}
