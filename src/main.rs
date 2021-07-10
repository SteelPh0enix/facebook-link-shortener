#![windows_subsystem = "windows"]

extern crate clipboard;
extern crate pretty_env_logger;

use log::{info, warn};
use std::{thread, time};

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

mod fburl;

fn main() {
    pretty_env_logger::init_timed();

    info!("Starting facebook link shortener...");

    let mut clip_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let sleep_time = time::Duration::from_secs(1);

    loop {
        thread::sleep(sleep_time);
        if let Ok(potential_url) = clip_ctx.get_contents() {
            if fburl::is_dirty_facebook_url(&potential_url) {
                info!("Found long facebook URL in clipboard: {}", potential_url);
                let new_url = fburl::shorten_facebook_url(&potential_url);
                info!("Shortened it to {}", new_url);
                match clip_ctx.set_contents(new_url) {
                    Ok(_) => info!("Short URL has been put in clipboard successfully!"),
                    Err(error) => warn!(
                        "Some error happened while putting the short URL in clipboard: {:?}",
                        error
                    ),
                }
            }
        }
    }
}
