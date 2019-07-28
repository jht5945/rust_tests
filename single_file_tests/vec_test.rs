
fn main() {
    let v = vec![1,3,5];
    for x in &v {
        println!("{}", x);
    }

    for (i, x) in v.iter().enumerate() {
        println!("{} -> {}", i, x);
    }
}

