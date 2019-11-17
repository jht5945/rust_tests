use std::{
    cmp::min,
    thread,
    time::Duration
};
use indicatif::{
    ProgressBar, 
    ProgressStyle
};

// https://crates.io/crates/indicatif
fn main() {
    let mut downloaded = 0;
    let total_size = 231231231;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(5));
    }

    pb.finish();
    // pb.finish_with_message("downloaded");

    let pb2 = ProgressBar::new(1024);
    for _ in 0..1024 {
        pb2.inc(1);
        thread::sleep(Duration::from_millis(5));
    }
    pb2.finish();
    // pb2.finish_with_message("done");
}
