# trs

Command-line interface for Google Translate.

## How to install

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

That's all.

## Usage

```bash
$ trs Hello world
# こんにちは世界

$ trs --from-target-language こんにちは
$ trs -f こんにちは
# Hello world

$ trs change-language zh
$ trs Hello
# 你好

$ trs -d dog
$ trs --definition dog
# Show formal definitions and example to use the word (of British English)

$ trs --version
$ trs --help
```
