use std::collections::{HashMap, HashSet};

fn main() {
	let ans = (2..1000).max_by_key(|x| cycle(*x)).unwrap();

	println!("{}", ans);
}

fn cycle(n: u64) -> u64 {
	let mut visit = HashMap::new();

	let mut cnt = 0u64;
	let mut res = 1u64;
	loop {
		res = 10 * res % n;
		cnt += 1;

		if res == 0 {
			return 0;
		} else if visit.contains_key(&res) {
			return cnt - visit[&res];
		}

		visit.insert(res, cnt);
	}
}

#[test]
fn cycle_test() {
	assert_eq!(cycle(2), 0);
	assert_eq!(cycle(3), 1);
	assert_eq!(cycle(4), 0);
	assert_eq!(cycle(5), 0);
	assert_eq!(cycle(6), 1);
	assert_eq!(cycle(7), 6);
	assert_eq!(cycle(8), 0);
	assert_eq!(cycle(9), 1);
}
