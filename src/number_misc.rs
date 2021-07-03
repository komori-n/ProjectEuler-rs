pub fn is_palindromic(n: u64) -> bool {
	let n_str = format!("{}", n);
	let rev_str: String = n_str.chars().rev().collect();

	n_str == rev_str
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
}
