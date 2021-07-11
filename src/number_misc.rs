use memoise::memoise;
use num::{Integer, One, Zero};
use std::cmp::PartialOrd;
use std::ops::{Div, Rem};

pub fn is_palindromic(n: u64) -> bool {
	let n_str = format!("{}", n);
	let rev_str: String = n_str.chars().rev().collect();

	n_str == rev_str
}

pub fn factorial<T: Integer + Zero + One + Clone>(n: T) -> T {
	if n == T::zero() {
		T::one()
	} else {
		n.clone() * factorial(n - T::one())
	}
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

const DIGIT1_LETTER: [&'static str; 20] = [
	"zero",
	"one",
	"two",
	"three",
	"four",
	"five",
	"six",
	"seven",
	"eight",
	"nine",
	"ten",
	"eleven",
	"twelve",
	"thirteen",
	"fourteen",
	"fifteen",
	"sixteen",
	"seventeen",
	"eighteen",
	"nineteen",
];

const DIGIT2_LETTER: [&'static str; 10] = [
	"zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

fn to_letters_below_100(n: u64) -> String {
	match n {
		i if i < 20 => DIGIT1_LETTER[i as usize].to_owned(),
		i if i % 10 == 0 => DIGIT2_LETTER[(i / 10) as usize].to_owned(),
		i if i < 100 => format!(
			"{}-{}",
			DIGIT2_LETTER[(i / 10) as usize],
			DIGIT1_LETTER[(i % 10) as usize]
		),
		_ => unreachable!(),
	}
}

pub fn to_letters(n: u64) -> String {
	match n {
		i if i < 100 => to_letters_below_100(n),
		i if i % 100 == 0 => format!("{} hundred", DIGIT1_LETTER[(i / 100) as usize]),
		i if i < 1000 => format!(
			"{} hundred and {}",
			DIGIT1_LETTER[(i / 100) as usize],
			to_letters_below_100(n % 100)
		),
		_ => unimplemented!(),
	}
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
	fn factorial_test() {
		assert_eq!(factorial(4), 24);
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

	#[test]
	fn to_letters_test() {
		assert_eq!(to_letters(0), String::from("zero"));
		assert_eq!(to_letters(1), String::from("one"));
		assert_eq!(to_letters(23), String::from("twenty-three"));
		assert_eq!(to_letters(34), String::from("thirty-four"));
		assert_eq!(to_letters(45), String::from("forty-five"));
		assert_eq!(to_letters(56), String::from("fifty-six"));
		assert_eq!(to_letters(67), String::from("sixty-seven"));
		assert_eq!(to_letters(78), String::from("seventy-eight"));
		assert_eq!(to_letters(89), String::from("eighty-nine"));
		assert_eq!(to_letters(91), String::from("ninety-one"));

		assert_eq!(
			to_letters(123),
			String::from("one hundred and twenty-three")
		);
		assert_eq!(
			to_letters(334),
			String::from("three hundred and thirty-four")
		);
	}
}
