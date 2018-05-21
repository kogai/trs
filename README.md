# TRS

CLI for English learners.

## Usage

[![asciicast](https://asciinema.org/a/Kb2wPXEYMpFKfv08LajsjS38E.png)](https://asciinema.org/a/Kb2wPXEYMpFKfv08LajsjS38E)

```bash
$ trs --to-target-language Hello world
$ trs -t Hello world
# こんにちは世界

$ trs --from-target-language こんにちは
$ trs -f こんにちは
# Hello world

$ trs --change-language zh # or simpliy `trs -C zh`
$ trs -t Hello # 你好
$ trs -f 你好 # Hello

$ trs -d dog
$ trs --definition dog
# Show formal definitions and example to use the word (of British English)

$ trs --version
$ trs --help
```

## How to install

1. Visit [release page](https://github.com/kogai/trs/releases) and download latest build.
1. Then set a $PATH to `/path/to/bin/{Darwin, Linux}`.
1. Give it a try.

## Build from source code

Set environemnt variables listed at `.envrc.template`

GOOGLE\_\* can get from here.
https://cloud.google.com/translate/docs/

OXFORD\_\* can get from here.
https://developer.oxforddictionaries.com/

And then, build it yourself using GNU Make.

```
$ make
```

Finally, set a $PATH to `/path/to/repository/bin/{Darwin, Linux}`

That is all.
