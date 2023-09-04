# Yet Another GUI Calculator Built on Rust

[![Rust Build and Test](https://github.com/codeitlikemiley/calc/actions/workflows/rust.yml/badge.svg)](https://github.com/codeitlikemiley/calc/actions/workflows/rust.yml)

<img width="277" alt="Screenshot 2023-09-04 at 7 51 45 AM" src="https://github.com/codeitlikemiley/calc/assets/28816690/3148628d-3e50-48d8-8ea7-576ebaf850d7">

## Installation of GUI App

- clone repo
```sh
git clone https://github.com/codeitlikemiley/calc
cd calc
git checkout gui
cargo build --release
# copy to your bin directory
cp target/release/calc ~/.local/bin
# copy to /Applications on Mac
```

## Installation of calc Terminal

```sh
git clone https://github.com/codeitlikemiley/calc
cd calc
git checkout cli
cargo build --release
# copy to your bin directory
cp target/release/calc ~/.local/bin
```

## Objectives

### CLI Mode

- [x] Basic arithmetic operations
- [x] Parentheses Operations
- [x] Chain Operations
- [x] Clear Screen
- [x] Exit
- [x] REPL mode
- [x] use Arrow Key Up and Down to navigate through history

### GUI  
- [x] Used Iced to build GUI App
- [x] Add Test 

