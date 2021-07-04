use memoise::memoise;

pub fn is_palindromic(n: u64) -> bool {
	let n_str = format!("{}", n);
	let rev_str: String = n_str.chars().rev().collect();

	n_str == rev_str
}

pub fn combination(n: u64, k: u64) -> u64 {
	combination_impl(n, k)
}

#[memoise(n <= 100, k <= 100)]
fn combination_impl(n: u64, k: u64) -> u64 {
	if k == 0 {
		return 1;
	}
	if n == 0 {
		return 0;
	}
	combination_impl(n - 1, k - 1) + combination_impl(n - 1, k)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn is_palindromic_test() {
		assert!(is_palindromic(2772));
		assert!(is_palindromic(272));
		assert!(!is_palindromic(334));
	}

	#[test]
	fn combination_test() {
		assert_eq!(combination(4, 2), 6);
		assert_eq!(combination(6, 2), 15);
	}
}
