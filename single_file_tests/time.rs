use std::time::SystemTime;

fn get_year_and_left_secs(secs_from_1970: u64) -> (u32, u64) {
    let mut t = secs_from_1970;
    let mut y = 1970;
    loop {
        let is_leap_year = y % 4 == 0 && y % 100 != 0;
        let secs_per_year = if is_leap_year { 366 } else { 365 } * 24 * 60 * 60;
        if t > secs_per_year {
            y += 1;
            t -= secs_per_year;
        } else {
            return (y, t);
        }
    }
}

fn main() {
    let secs_from_1970 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let (year_gmt, _) = get_year_and_left_secs(secs_from_1970);
    let (year_gmt8, _) = get_year_and_left_secs(secs_from_1970 + 8 * 60 * 60);

    println!("Secs from 1970: {}", secs_from_1970);
    println!("Year (GMT)    : {}", year_gmt);
    println!("Year (GMT+8)  : {}", year_gmt8);
}