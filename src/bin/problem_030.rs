use num::pow;

fn main() {
	println!("{}", solve(5));
}

fn solve(n: usize) -> u32 {
	(2..pow(10, n + 1))
		.filter_map(|i| {
			let sum = i
				.to_string()
				.chars()
				.map(|c| pow(c.to_digit(10).unwrap(), n))
				.sum::<u32>();

			if sum == i {
				Some(i)
			} else {
				None
			}
		})
		.sum::<u32>()
}

#[test]
fn solve_test() {
	assert_eq!(solve(4), 19316);
}
