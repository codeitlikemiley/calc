# Yet Another Calculator CLI built with Rust

## Installation

```bash
git clone https://github.com/codeitlikemiley/calc
cd calc
cargo build --release
# copy the executable to your bin folder
cp target/release/calc ~/.local/bin
# or to
cp target/release/calc /usr/local/bin
# make sure those paths are in your $PATH
```

## Usage:

```bash
calc
Enter an expression (e.g., 5 + 3) or 'exit|clear':
3 + 5 // 8
3 +   5 //8
3+3+3 //9
3+(2*2)-1 //6
```

## Objectives

- [x] Basic arithmetic operations
- [x] Parentheses Operations
- [x] Chain Operations
- [x] Clear Screen
- [x] Exit
- [x] REPL mode
- [x] use Arrow Key Up and Down to navigate through history

## Known Issue

when pressing up and down it prints `^[[A` and `^[[B` respectively
