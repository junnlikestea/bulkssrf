use std::sync::Arc;
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

async fn fetch(
    url: Arc<String>,
    header: &str,
    location: Arc<String>,
    timeout: u64,
    verbose: bool,
) -> Result<()> {
    let time = Duration::from_secs(timeout);

    let resp = reqwest::Client::new()
        .get(url.as_str())
        .timeout(time)
        .header(header, location.as_str())
        .send()
        .await;

    match resp {
        Ok(r) => println!("[{}] -> {}", r.status().as_str(), url),

        Err(e) => {
            if verbose {
                eprintln!("Requested: {} but was unreachable, with error: {}.", url, e)
            }
        }
    }
    Ok(())
}

async fn inject_headers(
    url: Arc<String>,
    location: Arc<String>,
    timeout: u64,
    verbose: bool,
) -> Result<()> {
    let mut tasks = Vec::new();

    for header in INJECTABLE_HEADERS.iter() {
        let url = url.clone();
        let location = location.clone();

        tasks.push(tokio::spawn(async move {
            if verbose {
                println!("Injecting:{} into {} -> {}", location, header, url);
            }

            fetch(url, header, location, timeout, verbose).await
        }))
    }

    for t in tasks {
        t.await?;
    }

    Ok(())
}

pub async fn run(urls: Vec<String>, location: String, timeout: u64, verbose: bool) {
    use futures::stream::StreamExt;

    const ACTIVE_REQUESTS: usize = 100;
    let shared_location = Arc::new(location);
    let responses = futures::stream::iter(urls.into_iter().map(|url| {
        let url = Arc::new(url);
        let loc = shared_location.clone();
        tokio::spawn(async move { inject_headers(url, loc, timeout, verbose).await })
    }))
    .buffer_unordered(ACTIVE_REQUESTS)
    .collect::<Vec<_>>();

    responses.await;
}
