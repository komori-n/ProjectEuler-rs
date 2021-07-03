use itertools::iproduct;

fn main() {
	iproduct!(1..1000u64, 1..1000u64)
		.filter(|(a, b)| a < b && a + 2 * b < 1000)
		.filter(|(a, b)| a * a + b * b == (1000 - a - b) * (1000 - a - b))
		.for_each(|(a, b)| println!("{} {} {} {}", a, b, 1000 - a - b, a * b * (1000 - a - b)));
}
