extern crate clipboard;

use std::{thread, time};

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

const FBID_PATTERNS: [&str; 1] = ["?__cft__"];

fn has_fbid(url: &str) -> bool {
    for pattern in FBID_PATTERNS {
        if url.find(pattern).is_some() {
            return true;
        }
    }
    false
}

fn is_long_facebook_url(potential_url: &str) -> bool {
    potential_url.find("facebook.com/").is_some() && has_fbid(potential_url)
}

fn shorten_facebook_url(url: &str) -> String {
    let mut clean_url = String::from(url);

    for pattern in FBID_PATTERNS {
        if let Some(index) = clean_url.find(pattern) {
            clean_url = clean_url.split_at(index).0.to_string();
        }
    }

    clean_url
}

fn main() {
    println!("Starting facebook link shortener daemon...");

    let mut clip_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let sleep_time = time::Duration::from_secs(1);

    loop {
        thread::sleep(sleep_time);
        if let Ok(potential_url) = clip_ctx.get_contents() {
            if is_long_facebook_url(&potential_url) {
                println!("Found long facebook URL in clipboard: {}", potential_url);
                let new_url = shorten_facebook_url(&potential_url);
                println!("Shortened it to {}", new_url);
                match clip_ctx.set_contents(new_url) {
                    Ok(_) => println!("Short URL has been put in clipboard successfully!"),
                    Err(error) => println!(
                        "Some error happened while putting the short URL in clipboard: {:?}",
                        error
                    ),
                }
            }
        }
    }
}
