# Yet Another Calculator Built on Rust

[![Rust Build and Test](https://github.com/codeitlikemiley/calc/actions/workflows/rust.yml/badge.svg)](https://github.com/codeitlikemiley/calc/actions/workflows/rust.yml)



## Installation of calc Terminal

```sh
git clone https://github.com/codeitlikemiley/calc
cd calc
git checkout cli
cargo build --release
# copy to your bin directory
cp target/release/calc ~/.local/bin/calc
```

## Usage of CLI

```bash
calc
Enter an expression (e.g., 5 + 3) or 'exit|clear':
3 + 5 // 8
3 +   5 //8
3+3+3 //9
3+(2*2)-1 //6
```

<img width="651" alt="Screenshot 2023-09-04 at 8 14 25 AM" src="https://github.com/codeitlikemiley/calc/assets/28816690/43304b02-2990-4658-a91f-eaa393e6c900">


## Installation of GUI App


```sh
git clone https://github.com/codeitlikemiley/calc
cd calc
git checkout gui
cargo build --release
# copy to your bin directory
cp target/release/calc ~/.local/bin/calg
# copy to /Applications on Mac
Note: if you will use both terminal and gui better alias gui to `calg`
```

## Usage of GUI Calculator

- invoke command `calg` to open it and start pressing buttons

<img width="277" alt="Screenshot 2023-09-04 at 7 51 45 AM" src="https://github.com/codeitlikemiley/calc/assets/28816690/3148628d-3e50-48d8-8ea7-576ebaf850d7">


## Objectives

### Build a Simple REPL CLI `calc`

- [x] Basic arithmetic operations
- [x] Parentheses Operations
- [x] Chain Operations
- [x] Clear Screen
- [x] Exit
- [x] REPL mode
- [x] use Arrow Key Up and Down to navigate through history

### Build a Simple GUI `calg`

- [x] Show a Simple GUI Calculator using Iced
- [x] Move Logic of Evaluating Expression to Update 
- [x] Add Test Basic Operations we can do

