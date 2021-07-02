pub struct Fibonacci {
	curr: u64,
	next: u64,
}

impl Iterator for Fibonacci {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		let new_next = self.curr + self.next;

		self.curr = self.next;
		self.next = new_next;

		Some(self.curr)
	}
}

impl Fibonacci {
	pub fn new(zero: u64, first: u64) -> Fibonacci {
		Fibonacci{curr: zero, next: first}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let mut fib = Fibonacci::new(0, 1);

		assert_eq!(fib.next().unwrap(), 1);
		assert_eq!(fib.next().unwrap(), 1);
		assert_eq!(fib.next().unwrap(), 2);
		assert_eq!(fib.next().unwrap(), 3);
		assert_eq!(fib.next().unwrap(), 5);
		assert_eq!(fib.next().unwrap(), 8);
	}
}