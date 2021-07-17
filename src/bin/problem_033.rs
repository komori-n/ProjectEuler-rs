use itertools::iproduct;
use num::integer::gcd;

fn main() {
	let ans = iproduct!(10..100, 10..100)
		.filter(|(a, b)| a < b)
		.filter(|(a, b)| a % 11 != 0 && (a % 10) == (b / 10) && a * (b % 10) == b * (a / 10))
		.fold((1, 1), |(nume, deno), (a, b)| (nume * a, deno * b));

	println!("{}", ans.1 / gcd(ans.0, ans.1));
}
