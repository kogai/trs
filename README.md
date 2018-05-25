# TRS

CLI for English learners.

## Usage

[![asciicast](https://asciinema.org/a/Kb2wPXEYMpFKfv08LajsjS38E.png)](https://asciinema.org/a/Kb2wPXEYMpFKfv08LajsjS38E)

## How to install

1.  `curl https://raw.githubusercontent.com/kogai/trs/master/install.sh -Ssf | sh`
1.  Then set a $PATH to `~/bin`.
1.  Give it a try.

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
