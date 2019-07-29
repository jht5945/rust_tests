
fn main() {
    let v = vec![1, 3, 5];
    println!("For each print:");
    for x in &v {
        println!("{}", x);
    }

    println!("Enumerate for each print:");
    for (i, x) in v.iter().enumerate() {
        println!("{} -> {}", i, x);
    }

    println!("Pop print:");
    let mut v2 = vec![2, 4, 6];
    while let Some(x) = v2.pop() {
        println!("{}", x);
    }
}

