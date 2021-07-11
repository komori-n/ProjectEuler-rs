use project_euler_rs::permutation::next_permutation;

fn main() {
	let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let len = arr.len();

	for _i in 0..(1_000_000 - 1) {
		if !next_permutation(&mut arr, len) {
			break;
		}
	}

	println!("{:?}", arr);
}
