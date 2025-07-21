# minigrep

A simple cli tool like grep with limited features, an exercise of rust book documents
which also contains step-by-step implementation:

https://doc.rust-lang.org/book/

## Clone and build
Clone repository:
```shell
git clone https://github.com/ArshiaYousefnia/minigrep.git
```
Build via cargo:
```shell
cd minigrep
cargo build --release
```

## Usage and features
Add executable to $PATH (optional):
```shell
./target/release/minigrep <query> <filename>
```
query is a string which minigrep will look for exact match in contents of filename.

Search is case-sensitive by default, but you can set $IGNORE_CASE, then search will be 
case-insensitive:
```shell
export IGNORE_CASE=1
unset IGNORE_CASE
```

