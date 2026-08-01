# CNGREP(CG)
CNGREP is a simple command-line text search tool written in Rust.
It is designed as a learning project like `grep``ripgrep`

## Features
Currently, cngrep supports:
- Search text in a single or multiple file and recursive directory
- Search from standard input
- Options include normal and count only mode

## Build
Build the project with cargo `cargo build`
For a release build `cargo build --release`
The executable is currently named `cg`

## Usage
The basic command syntax is `cg [OPTIONS] <PATTERN> [PATH]`
The command accepts:
- `PATTERN`: text to search for
- `Path`: file or directory to search
if no path is provided and command stdin, `cg` searches the current 
directory. Search stdin content if provided command stdin.
- `OPTIONS`: optional software running modes