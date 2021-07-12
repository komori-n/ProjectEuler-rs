use std::collections::HashSet;
use std::iter::FromIterator;

use itertools::iproduct;
use num::BigInt;

fn main() {
	let iter = iproduct!(2..101, 2..101).map(|(a, b)| BigInt::from(a).pow(b));
	let set = HashSet::<_>::from_iter(iter);

	println!("{}", set.len());
}
