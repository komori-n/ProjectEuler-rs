use memoise::memoise;
use num::Zero;
use std::cmp::PartialOrd;
use std::ops::{Div, Rem};

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

pub struct EachDigit<T> {
	remain: T,
}

impl<T> Iterator for EachDigit<T>
where
	T: Zero + PartialOrd + Rem<Output = T> + Div<Output = T> + From<i32> + Clone,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.remain > T::zero() {
			let digit = self.remain.clone() % T::from(10);
			self.remain = self.remain.clone() / T::from(10);

			Some(digit)
		} else {
			None
		}
	}
}

impl<T> EachDigit<T> {
	pub fn new(n: T) -> Self {
		EachDigit { remain: n }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use num::BigInt;

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

	#[test]
	fn each_digit() {
		assert_eq!(EachDigit::new(334).collect::<Vec<_>>(), vec![4, 3, 3]);
		assert_eq!(
			EachDigit::new(BigInt::from(334)).collect::<Vec<_>>(),
			vec![4.into(), 3.into(), 3.into()]
		);
	}
}
