use regex::Regex;

const TO_SEARCH: &str = "
On 2010-03-14, foo happened. On 2014-10-14, bar happened.
";

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    
    for caps in re.captures_iter(TO_SEARCH) {
        println!("year: {}, month: {}, day: {}",
             caps.get(1).unwrap().as_str(),
             caps.get(2).unwrap().as_str(),
             caps.get(3).unwrap().as_str());
    }

    let re2 = Regex::new(r"(\d+)").unwrap();
    println!("{}", re2.replace_all("Hello 100, 200", | caps: &regex::Captures | {
        "(".to_owned() + &caps[1] + ")"
    }));
    println!("{}", re2.replace_all("Hello 100, 200", | caps: &regex::Captures<'_> | {
        "[".to_owned() + &caps[1] + "]"
    }));
}
