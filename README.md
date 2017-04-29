# trs

Translate words via Google translate API in terminal.

## Usage

1. Create Google translate API KEY in Google cloud platform
1. Set API KEY as environment variable "GOOGLE_CLOUD_PLATFORM_API_KEY"
1. Build executable with `cargo build --release`

```bash
./target/release/trs -q Hello world
// こんにちは世界

.target/release/trs -q こんにちは 世界 -t en
// Hello world

./target/release/trs --version 
./target/release/trs --help 
``` 