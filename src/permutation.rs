//! ref: https://qiita.com/cielavenir/items/cec1812999547909a9e4#rust

fn reverse<T>(a: &mut [T], start_: usize, size: usize) {
	let mut start = start_;
	let mut end = start + size - 1;
	while start < end {
		{
			let p1: *mut T = &mut a[start];
			let p2: *mut T = &mut a[end];
			unsafe {
				p1.swap(p2);
			}
		}
		end -= 1;
		start += 1;
	}
}

pub fn next_permutation<T: std::cmp::PartialOrd>(a: &mut [T], n: usize) -> bool {
	if n < 0 || a.len() < n {
		return false;
	}
	reverse(a, n, a.len() - n);
	let mut i = a.len() - 2;
	loop {
		if a[i] < a[i + 1] {
			break;
		}
		if i == 0 {
			reverse(a, 0, a.len());
			return false;
		}
		i -= 1;
	}
	let k = i;
	i = a.len() - 1;
	while i >= k + 1 {
		if a[k] < a[i] {
			break;
		}
		i -= 1;
	}
	let l = i;
	{
		let p1: *mut T = &mut a[k];
		let p2: *mut T = &mut a[l];
		unsafe {
			p1.swap(p2);
		}
	}
	reverse(a, k + 1, a.len() - (k + 1));
	return true;
}
