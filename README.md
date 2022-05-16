# random number genorter cli

## install

### clone the repo first

```
git clone https://github.com/Senpai-10/ran.git &&
cd ran &&
```

### install the binary

$ `cargo install --path .`

### or

$ `make install`

## help command

```
USAGE:
    ran [OPTIONS]

OPTIONS:
    -a, --delimiter-after          Print delimiter after number
    -b, --delimiter-before         Print delimiter before number
    -c, --count <COUNT>            Number of generated numbers [default: 1]
    -d, --delimiter <DELIMITER>    Change inline print delimiter [default: " "]
    -h, --help                     Print help information
    -i, --inline                   Inline print all generated numbers
    -m, --min <MIN>                [default: 1]
    -M, --max <MAX>                [default: 100]
    -V, --version                  Print version information
```
