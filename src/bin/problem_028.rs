fn main() {
	println!("{}", solve(1001));
}

fn solve(n: u64) -> u64 {
	if n == 1 {
		return 1;
	} else {
		let ru = n * n;
		let lu = n * n - (n - 1);
		let ld = lu - (n - 1);
		let rd = ld - (n - 1);

		return ru + lu + ld + rd + solve(n - 2);
	}
}

#[test]
fn solve_test() {
	assert_eq!(solve(5), 101);
}
