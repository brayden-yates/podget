use reqwest::blocking::get;
use rss::Channel;
use std::error::Error;
use std::fs::{self, File};
use std::io::copy;
use std::env;
use std::path::{Path, PathBuf};
use home::home_dir;

fn download_file(url: &str, save_dir: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let response = get(url)?;
    let file_path = Path::new(save_dir).join(filename);
    let mut file = File::create(&file_path)?;
    copy(&mut response.bytes()?.as_ref(), &mut file)?;
    Ok(())
}

fn download_podcasts(rss_url: &str, save_dir: &str) -> Result<(), Box<dyn Error>> {
    let body = get(rss_url)?.text()?;
    let channel = Channel::read_from(body.as_bytes())?;

    fs::create_dir_all(save_dir)?;

    for item in channel.items() {
        if let Some(enclosure) = item.enclosure() {
            let url = enclosure.url();
            let title = item.title().unwrap_or("unknown").replace('/', "_");
            let filename = format!("{}.mp3", title);

            println!("Downloading {} from {}", filename, url);
            download_file(url, save_dir, &filename)?;
        }
    }

    Ok(())
}

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some(home) = home_dir() {
            let path_without_tilde = &path[1..];
            return home.join(path_without_tilde);
        }
    }
    Path::new(path).to_path_buf() 
}

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <RSS_URL> [SAVE_DIR]", args[0]);
        std::process::exit(1);
    }

    let rss_url = &args[1];
    let save_dir = if args.len() > 2 { &args[2] } else { "." }; 
    
    let expanded_save_dir = expand_tilde(save_dir);

    download_podcasts(rss_url, expanded_save_dir.to_str().unwrap())?;
    Ok(())
}
