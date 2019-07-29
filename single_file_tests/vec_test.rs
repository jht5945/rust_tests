
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

    println!("Map:");
    let v3 = vec![1, 2, 3];
    let v4 = v3.iter().map(|&x| x * 2).collect::<Vec<_>>();
    println!("{:?} -> {:?}", v3, v4);
}

