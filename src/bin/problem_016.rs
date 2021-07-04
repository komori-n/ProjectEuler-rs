use num::BigInt;
use project_euler_rs::number_misc::EachDigit;

fn main() {
	let n = BigInt::from(2).pow(1000);

	let ans = EachDigit::new(n).sum::<BigInt>();

	println!("{}", ans);
}
