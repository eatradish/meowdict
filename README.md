# meowdict
CLI Web client for moedict.tw

## Screenshot

![screenshot](https://raw.githubusercontent.com/eatradish/meowdict/master/screenshot/Screenshot_20211022_113701.png)

## How to find me?
- Github Issue
- Telegram Group: https://t.me/meodict_dev

## Installation
If you use AOSC OS:

```
sudo apt install meowdict
```

Else:

```
$ cargo build --release
# install -vm755 target/release/meowdict /usr/local/bin/meowdict
```

## Dependencies
Building:
- Rust w/ Cargo
- C compiler
- make (when GCC LTO is used, not needed for Clang)

Runtime:
- Glibc
- OpenSSL
- OpenCC (>= 1.1.2)


## Usage

```
$ ./target/debug/meowdict help
meowdict 0.10.4-alpha.0
Mag Mell
Search chinese keyword from moedict.tw

USAGE:
    meowdict [FLAGS] [INPUT]... [SUBCOMMAND]

FLAGS:
    -h, --help               Prints help information
    -i, --input-s2t          Convert input to traditional Chinese and search
        --input-s2t-mode     Open console with input-s2t mode
        --no-color-output    Print result with no color
    -r, --result-t2s         Convert result to Simplified Chinese to display
        --result-t2s-mode    Open console with result-t2s mode
    -V, --version            Prints version information

ARGS:
    <INPUT>...    Input the keyword to use

SUBCOMMANDS:
    help         Prints this message or the help of the given subcommand(s)
    json         Print result to JSON output
    jyutping     Get word jyutping
    random       search random word
    show         Get dict result
    terminal     Open meowdict terminal
    translate    Get word translation
```