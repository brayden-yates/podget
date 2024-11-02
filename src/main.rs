use reqwest::blocking::get;
use rss::Channel;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::env;
fn download_file(url: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let response = get(url)?;
    let mut file = File::create(filename)?;
    copy(&mut response.bytes()?.as_ref(), &mut file)?;
    Ok(())
}

fn download_podcasts(rss_url: &str) -> Result<(), Box<dyn Error>> {
    let body = get(rss_url)?.text()?;
    let channel = Channel::read_from(body.as_bytes())?;

    for item in channel.items() {
        if let Some(enclosure) = item.enclosure() {
            let url = enclosure.url();
            let title = item.title().unwrap_or("unknown").replace('/', "_");
            let filename = format!("{}.mp3", title);

            println!("Downloading {} from {}", filename, url);
            download_file(url, &filename)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Ensure that the user has provided a URL
    if args.len() < 2 {
        eprintln!("Usage: {} <RSS_URL>", args[0]);
        std::process::exit(1);
    }

    let rss_url = &args[1];
    download_podcasts(rss_url)?;
    Ok(())
}

