pub struct Triangular {
	curr: u64,
}

impl Iterator for Triangular {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		let ret = self.curr * (self.curr + 1) / 2;
		self.curr += 1;

		Some(ret)
	}
}

impl Triangular {
	pub fn new() -> Self {
		Triangular { curr: 1 }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let mut tri = Triangular::new();

		assert_eq!(tri.next().unwrap(), 1);
		assert_eq!(tri.next().unwrap(), 3);
		assert_eq!(tri.next().unwrap(), 6);
		assert_eq!(tri.next().unwrap(), 10);
		assert_eq!(tri.next().unwrap(), 15);
		assert_eq!(tri.next().unwrap(), 21);
	}
}
