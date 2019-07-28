extern crate otpauth;
extern crate time;

use otpauth::TOTP;

fn main() {
    let auth = TOTP::new("python");
    let timestamp1 = time::now().to_timespec().sec as usize;
    let code = auth.generate(30, timestamp1);
    let timestamp2 = time::now().to_timespec().sec as usize;
    println!("code: {}", code);
    assert_eq!(true, auth.verify(code, 30, timestamp2));
}