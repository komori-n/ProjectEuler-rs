pub struct Collatz {
	curr: u64,
}

impl Iterator for Collatz {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		if self.curr != 1 {
			self.curr = collatz_next(self.curr);
			Some(self.curr)
		} else {
			None
		}
	}
}

impl Collatz {
	pub fn new(n: u64) -> Self {
		Collatz { curr: n }
	}
}

pub fn collatz_next(n: u64) -> u64 {
	if n % 2 == 0 {
		n / 2
	} else {
		3 * n + 1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn collatz_test() {
		let collatz = Collatz::new(13);

		assert_eq!(
			collatz.collect::<Vec<_>>(),
			vec![40, 20, 10, 5, 16, 8, 4, 2, 1]
		);
	}
}
