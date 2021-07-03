pub struct Primes {
	list: Vec<u64>,
}

impl Primes {
	pub fn new(max_searched: u64) -> Self {
		let mut is_prime = vec![true; (max_searched + 1) as usize];
		let mut prime_list = Vec::new();

		is_prime[0] = false;
		is_prime[1] = false;

		for x in 2..max_searched {
			if is_prime[x as usize] {
				prime_list.push(x);
				(2u64..)
					.take_while(|i| *i * x <= max_searched)
					.for_each(|i| {
						is_prime[(i * x) as usize] = false;
					});
			}
		}

		Primes { list: prime_list }
	}

	pub fn iter(&self) -> std::slice::Iter<u64> {
		self.list.iter()
	}

	pub fn factorize(&self, mut n: u64) -> Vec<u64> {
		let mut factors = Vec::new();

		for &p in self.iter() {
			if p * p > n {
				break;
			}

			while n % p == 0 {
				factors.push(p);
				n /= p;
			}
		}

		if n > 1 {
			factors.push(n);
		}

		factors
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let primes = Primes::new(10);

		assert_eq!(primes.iter().collect::<Vec<_>>(), vec![&2, &3, &5, &7,]);
	}

	#[test]
	fn factors() {
		let primes = Primes::new(100);

		let factors = primes.factorize(24);

		assert_eq!(factors, vec![2, 2, 2, 3]);
	}
}
