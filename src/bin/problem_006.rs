fn main() {
	println!("{}", solve(100));
}

fn solve(n: u64) -> u64 {
	let (square_sum, sum) =
		(1..n + 1).fold((0, 0), |(square_sum, sum), x| (square_sum + x * x, sum + x));

	sum * sum - square_sum
}

#[test]
fn solve_test() {
	assert_eq!(solve(10), 2640);
}
