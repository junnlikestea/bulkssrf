extern crate bulkssrf;
use bulkssrf::run;
use bulkssrf::Result;
use clap::{App, Arg};
use std::fs;
use std::io::{self, Read};

#[tokio::main]
async fn main() -> Result<()> {
    let args = create_clap_app("v0.1.0");
    let matches = args.get_matches();
    let mut urls: Vec<String> = Vec::new();
    let location = matches.value_of("location").unwrap();
    let timeout: u64 = matches.value_of("timeout").unwrap().parse()?;

    if matches.is_present("file") {
        let input = matches.value_of("input").unwrap();
        let contents = fs::read_to_string(input)?;
        urls = contents.lines().map(|l| l.to_string()).collect();
    } else {
        urls = read_stdin()?;
    }
    run(urls, location.to_string(), timeout).await;
    Ok(())
}

fn create_clap_app(version: &str) -> clap::App {
    // Add support to not include subdomains.
    App::new("rs")
        .version(version)
        .about("Test for SSRF by injecting a location into headers.")
        .usage("rs <urls> -l <location> or rs -location")
        .arg(Arg::with_name("input").index(1).required(false))
        .arg(
            Arg::with_name("file")
                .help("rs -f <urls.txt> -l <location>")
                .short("f")
                .long("file"),
        )
        .arg(
            Arg::with_name("location")
                .help("The server address you want the ssrf to hit")
                .takes_value(true)
                .required(true)
                .short("l")
                .long("location"),
        )
        .arg(
            Arg::with_name("timeout")
                .help("the connection timeout i.e. the time to wait for a response body.")
                .short("t")
                .long("timeout")
                .default_value("4")
                .takes_value(true),
        )
}

fn read_stdin() -> Result<Vec<String>> {
    let mut buffer = String::new();
    let mut res = Vec::new();
    io::stdin().read_to_string(&mut buffer)?;
    for line in buffer.split_whitespace() {
        res.push(line.to_string())
    }
    Ok(res)
}
