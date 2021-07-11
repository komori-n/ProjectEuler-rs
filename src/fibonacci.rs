use num::Num;

pub struct Fibonacci<T: Num + Clone> {
	curr: T,
	next: T,
}

impl<T: Num + Clone> Iterator for Fibonacci<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let new_next = self.curr.clone() + self.next.clone();

		self.curr = self.next.clone();
		self.next = new_next;

		Some(self.curr.clone())
	}
}

impl<T: Num + Clone> Fibonacci<T> {
	pub fn new(zero: T, first: T) -> Self {
		Self {
			curr: zero,
			next: first,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let mut fib = Fibonacci::<u64>::new(0, 1);

		assert_eq!(fib.next().unwrap(), 1);
		assert_eq!(fib.next().unwrap(), 1);
		assert_eq!(fib.next().unwrap(), 2);
		assert_eq!(fib.next().unwrap(), 3);
		assert_eq!(fib.next().unwrap(), 5);
		assert_eq!(fib.next().unwrap(), 8);
	}
}
