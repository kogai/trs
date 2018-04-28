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
$ trs -q Hello world
# こんにちは世界

$ trs -q こんにちは 世界 -t en
# Hello world

$ trs -d dog
# Show definitions and example to use the word

$ trs --version
$ trs --help
```
