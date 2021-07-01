fn main() {
	println!("{}", solve(1000));
}

fn solve(n: u64) -> u64 {
	(1..n)
		.filter(|x| x % 3 == 0 || x % 5 == 0)
		.sum()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn basic() {
		assert_eq!(solve(10), 23);
	}
}