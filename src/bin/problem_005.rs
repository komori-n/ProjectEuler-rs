use num::integer::lcm;

fn main() {
	let ans = (1..20).fold(1, |curr, x| lcm(curr, x));

	println!("{}", ans);
}
