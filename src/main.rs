extern crate clipboard;

use std::{thread, time};

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

mod fburl;

fn main() {
    println!("Starting facebook link shortener...");

    let mut clip_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let sleep_time = time::Duration::from_secs(1);

    loop {
        thread::sleep(sleep_time);
        if let Ok(potential_url) = clip_ctx.get_contents() {
            if fburl::is_dirty_facebook_url(&potential_url) {
                println!("Found long facebook URL in clipboard: {}", potential_url);
                let new_url = fburl::shorten_facebook_url(&potential_url);
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
