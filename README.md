# Facebook link shortener

This is a simple program, written in Rust, that runs in background and checks every second for Facebook URL in clipboard. If it detects it, then it removes the cft/fbclid part, leaving the short URL in clipboard.

## Building

Simply, run `cargo build --release`

## Running

Probably the best way to run this program is either adding it to autostart, or creating a system service to automatically start it with OS.
Otherwise, you can just `cargo run` it for testing - the output log will be in `stdout`.

## Compatibility

Pretty sure it'll run on Linux and Windows, not sure about MacOS. The compatibility is restricted by `clipboard` package support.

## Contributing / Issues

Just make a PR, i'll check it out in free time
If you'll find an issue, just report it in Issues.

## License

See `LICENSE` file.
Spoiler: it's MIT License.
