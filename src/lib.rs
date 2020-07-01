use futures::future::join_all;
use std::time::Duration;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

const INJECTABLE_HEADERS: &[&str] = &[
    "Proxy-Host",
    "Request-Uri",
    "X-Forwarded",
    "X-Forwarded-By",
    "X-Forwarded-For",
    "X-Forwarded-For-Original",
    "X-Forwarded-Host",
    "X-Forwarded-Server",
    "X-Forwarder-For",
    "X-Forward-For",
    "Base-Url",
    "Http-Url",
    "Proxy-Url",
    "Redirect",
    "Real-Ip",
    "Referer",
    "Referer",
    "Referrer",
    "Refferer",
    "Uri",
    "Url",
    "X-Host",
    "X-Http-Destinationurl",
    "X-Http-Host-Override",
    "X-Original-Remote-Addr",
    "X-Original-Url",
    "X-Proxy-Url",
    "X-Rewrite-Url",
    "X-Real-Ip",
    "X-Remote-Addr",
];

struct Response {
    url: String,
    status: u16,
}

impl Response {
    fn new(url: String, status: u16) -> Self {
        Response { url, status }
    }
}

async fn fetch(url: String, header: &str, location: String, timeout: u64) -> Result<Response> {
    let time = Duration::from_secs(timeout);
    let resp = reqwest::Client::new()
        .get(&url)
        .timeout(time)
        .header(header, location)
        .send()
        .await?;
    println!("[{}] -> {}", resp.status().as_u16(), &url);

    Ok(Response::new(url.to_owned(), resp.status().as_u16()))
}

async fn inject_headers(url: String, location: String, timeout: u64) -> Result<()> {
    let mut tasks = Vec::new();
    for header in INJECTABLE_HEADERS.iter() {
        let l = location.clone();
        let u = url.clone();

        tasks.push(tokio::spawn(
            async move { fetch(u, header, l, timeout).await },
        ))
    }

    join_all(tasks).await;
    Ok(())
}

pub async fn run(urls: Vec<String>, location: String, timeout: u64) {
    const ACTIVE_REQUESTS: usize = 100;

    use futures::stream::StreamExt;
    let responses = futures::stream::iter(urls.into_iter().map(|url| {
        let l = location.clone();
        tokio::spawn(async move { inject_headers(url, l, timeout).await })
    }))
    .buffer_unordered(ACTIVE_REQUESTS)
    .collect::<Vec<_>>();

    responses.await;
}
