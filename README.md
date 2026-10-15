# Manticore Verilog to MASM

[![Rust](https://img.shields.io/badge/Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-TBD-lightgrey)](#license)

A Rust command-line tool that automates the Manticore/Yosys frontend and generates Manticore Assembly (`main.masm`) from Verilog or SystemVerilog designs.

## Overview

This tool takes one or more Verilog/SystemVerilog files as command-line arguments, runs the existing `v2masmyosys` frontend, and generates a `main.masm` netlist assembly file.

```text
Verilog/SystemVerilog → Rust tool → v2masmyosys → main.masm
```

The existing Manticore/Yosys frontend is reused. This project focuses on automating the frontend from Rust.

## Features

- Accepts one or more Verilog/SystemVerilog files as arguments
- Checks that every input file exists before running anything
- Finds the frontend through an environment variable or your `PATH`, with no hardcoded paths
- Runs the frontend in the current directory, or in a directory you choose with `--working-dir`
- Executes the required Yosys and Manticore passes
- Generates `main.masm`
- Shows the frontend's output live in the terminal
- Exits with a non-zero code on failure, so it can be used in scripts

## Requirements

- Rust and Cargo
- A built Manticore frontend containing `v2masmyosys`
- Verilog or SystemVerilog source files

## Installation

Clone the repository:

```bash
git clone https://github.com/hannahkarian/hana-manticore.git
cd hana-manticore
```

Build the project:

```bash
cargo build
```

## Configuration

The tool needs to know where `v2masmyosys` is. This is a one-time setup on each machine, and you can do it in either of two ways.

### Option 1: Set `MANTICORE_YOSYS_FRONTEND` (recommended)

Add this line to the end of your `~/.bashrc` (or `~/.zshrc` if you use zsh), using the path to your own copy of the frontend:

```bash
export MANTICORE_YOSYS_FRONTEND=/path/to/manticore-frontend/v2masmyosys
```

Then open a new terminal, or run `source ~/.bashrc`. Check that it worked:

```bash
echo $MANTICORE_YOSYS_FRONTEND
```

### Option 2: Put `v2masmyosys` on your `PATH`

If the variable is not set, the tool runs `v2masmyosys` by name and lets the system find it on your `PATH`. Add the frontend's folder to your `PATH` in `~/.bashrc`:

```bash
export PATH="$PATH:/path/to/manticore-frontend"
```

Check that it worked:

```bash
which v2masmyosys
```

There is no need to copy `v2masmyosys` into this project. Leaving it where it was built also keeps any support files it relies on next to it.

## Usage

```text
hana-manticore [--working-dir DIR] FILE.v [FILE.v ...]
```

With Cargo, arguments for the program go after `--`:

```bash
cargo run -- top.sv counter.sv
```

All files are passed to Yosys together as one design. The frontend runs in the current directory, so `main.masm` is created there:

```text
Created: /home/you/my-design/main.masm
You can now parse this file:
/home/you/my-design/main.masm
```

To create `main.masm` somewhere else, use `--working-dir`:

```bash
cargo run -- --working-dir build/ top.sv counter.sv
```

Relative input paths are always resolved from where you run the command, even when `--working-dir` points somewhere else.

## Compilation Pipeline

The tool performs the following steps:

1. Reads the Verilog/SystemVerilog input files.
2. Checks and resolves the module hierarchy.
3. Converts procedural logic into a netlist.
4. Optimizes and cleans the netlist.
5. Processes memories and flip-flops.
6. Applies Manticore-specific transformations and checks.
7. Prints design statistics.
8. Writes the final netlist assembly to `main.masm`.

## Project Structure

```text
.
├── Cargo.toml       # Rust package configuration
├── Cargo.lock       # Locked dependency versions
├── README.md        # Project documentation
└── src
    └── main.rs      # Main Rust program
```

`src/main.rs` is split into small functions, each with one job:

| Function | Purpose |
| --- | --- |
| `frontend_bin` | Returns `$MANTICORE_YOSYS_FRONTEND`, or `v2masmyosys` if it is not set |
| `build_yosys_commands` | Builds the Yosys command string passed with `-p` |
| `run_manticore_frontend` | Runs the frontend in the working directory and checks that `main.masm` was created |
| `run` | Reads the command line arguments, checks that each file exists, makes the paths absolute and picks the working directory |
| `main` | Calls `run`, prints the path of `main.masm`, and reports errors |

## Troubleshooting

**`could not start v2masmyosys: No such file or directory`**
The tool cannot find the frontend. Follow the steps in [Configuration](#configuration), then open a new terminal.

**`No Verilog files were provided.`**
No input files were given. Pass at least one Verilog/SystemVerilog file after `cargo run --`.

**`File does not exist: ...`**
One of the input paths is wrong. Paths are relative to the directory you run the command from.

**`The Manticore frontend failed.`**
Yosys reported an error. Its output is printed just above this message.

## Development Commands

Format the code:

```bash
cargo fmt
```

Check whether the project compiles:

```bash
cargo check
```

Run the application:

```bash
cargo run -- design.sv
```

## Generated Files

The following files should not normally be committed:

```gitignore
/target/
main.masm
```

The `target` directory contains Rust build output. The `main.masm` file is generated from the Verilog input and can be recreated by running the program.

## Current Status

Completed:

- Manticore/Yosys frontend setup
- Rust integration using `std::process::Command`
- Command-line input of Verilog/SystemVerilog files
- Configurable frontend location and working directory
- Automated generation of `main.masm`
