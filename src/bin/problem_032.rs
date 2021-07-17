use std::collections::HashSet;

use itertools::{iproduct, Itertools};

fn main() {
	let mut ans = HashSet::new();
	// 1-digit x (<=4)-digit
	iproduct!(1..10, 1..10_000)
		.filter(|(i, j)| is_pandigital(*i, *j))
		.for_each(|(i, j)| {
			ans.insert(i * j);
		});

	// 2-digit x (<=3)-digit
	iproduct!(10..100, 1..1_000)
		.filter(|(i, j)| is_pandigital(*i, *j))
		.for_each(|(i, j)| {
			ans.insert(i * j);
		});

	println!("{}", ans.iter().sum::<i64>());
}

fn is_pandigital(n: i64, m: i64) -> bool {
	let prod = n * m;
	format!("{}{}{}", n, m, prod)
		.chars()
		.sorted()
		.collect::<String>()
		== String::from("123456789")
}
