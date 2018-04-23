# trs

Command-line interface for Google Translate.

## How to install

Set environemnt variables listed at `.envrc.template`

GOOGLE_* can get from here.
https://cloud.google.com/translate/docs/

OXFORD_* can get from here.
https://developer.oxforddictionaries.com/

And then, build it yourself using GNU Make.

```
$ cargo build --release
$ cp target/release/trs bin/
```

Finally, set a $PATH to `/path/to/repository/bin`

That's all.

## Usage

1. Create Google translate API KEY in Google cloud platform
1. Set API KEY as environment variable "GOOGLE_CLOUD_PLATFORM_API_KEY"
1. Build executable with `cargo build --release`

```bash
./target/release/trs -q Hello world
// こんにちは世界

./target/release/trs -q こんにちは 世界 -t en
// Hello world

./target/release/trs --version 
./target/release/trs --help 
``` 
