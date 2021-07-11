use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};

fn main() {
	let start = Utc.ymd(1901, 1, 1);
	let end = Utc.ymd(2000, 12, 31);

	let mut cnt = 0;
	let mut dt = start;
	while dt <= end {
		if dt.day() == 1 && dt.weekday() == Weekday::Sun {
			cnt += 1;
		}
		dt = dt + Duration::days(1);
	}

	println!("{}", cnt);
}
