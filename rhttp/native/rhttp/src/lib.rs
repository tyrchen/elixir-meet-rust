use tokio::runtime::Runtime;

#[rustler::nif(schedule = "DirtyIo")]
fn get(url: String) -> String {
    let mut rt = Runtime::new().unwrap();
    let result = rt.block_on(async { http_get(&url).await });
    result
}

async fn http_get(url: &str) -> String {
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

rustler::init!("Elixir.Rhttp", [get]);
