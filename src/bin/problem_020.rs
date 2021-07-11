use num::BigInt;
use project_euler_rs::number_misc::factorial;

fn main() {
	println!("{}", solve(100));
}

fn solve(n: u64) -> u64 {
	factorial(BigInt::from(n))
		.to_string()
		.chars()
		.map(|c| c.to_digit(10).unwrap() as u64)
		.sum::<u64>()
}

#[test]
fn basic() {
	assert_eq!(solve(10), 27);
}
