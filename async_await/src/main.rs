

fn main() {
    futures::executor::block_on(get_all());
}

async fn get_all() {
    let mut v = vec![];
    for _ in 0..10 {
        v.push(get_url("https://www.baidu.com/"));
    }
    
    futures::future::join_all(v).await;
}

async fn get_url(url: &str) {
    println!("Start get: {}", url);
    let r = isahc::get_async(url).await;

    match r {
        Err(e) => println!("Get response error: {}", e),
        Ok(o) =>  println!("Response: {:?}", o.status()),
    };
}