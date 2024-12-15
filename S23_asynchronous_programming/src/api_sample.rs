use tokio::task;

pub async fn run() {
    let task_a = task::spawn(fetch_data_async(
        "https://jsonplaceholder.typicode.com/posts/1",
    ));
    let task_b = task::spawn(fetch_data_async(
        "https://jsonplaceholder.typicode.com/posts/2",
    ));
    let task_c = task::spawn(fetch_data_async(
        "https://jsonplaceholder.typicode.com/posts/3",
    ));

    let (res_a, res_b, res_c) = tokio::join!(task_a, task_b, task_c);

    match (res_a, res_b, res_c) {
        (Ok(a), Ok(b), Ok(c)) => {
            println!("{:?}", a);
            println!("{:?}", b);
            println!("{:?}", c);
        }
        _ => println!("Failed to fetch data"),
    }
}

async fn fetch_data_async(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}
