# Manticore Verilog to MASM

[![Rust](https://img.shields.io/badge/Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-TBD-lightgrey)](#license)

A Rust command-line tool that automates the Manticore/Yosys frontend and generates Manticore Assembly (`main.masm`) from Verilog or SystemVerilog designs.

## Overview

This tool collects one or more Verilog/SystemVerilog file paths, runs the existing `v2masmyosys` frontend, and generates a `main.masm` netlist assembly file.

```text
Verilog/SystemVerilog → Rust tool → v2masmyosys → main.masm
```

The existing Manticore/Yosys frontend is reused. This project focuses on automating the frontend from Rust.

## Features

- Accepts one or more Verilog/SystemVerilog files
- Validates entered file paths
- Runs the Manticore/Yosys frontend automatically
- Executes the required Yosys and Manticore passes
- Generates `main.masm`
- Displays frontend output and errors

## Requirements

- Rust and Cargo
- A built Manticore frontend containing `v2masmyosys`
- Verilog or SystemVerilog source files

## Installation

Clone the repository:

```bash
git clone https://github.com/hannahkarian/manticore-verilog-to-masm.git
cd manticore-verilog-to-masm
```

Build the project:

```bash
cargo build
```

## Configuration

The path to `v2masmyosys` is currently configured in `src/main.rs`:

```rust
let frontend_binary =
    Path::new("/home/hana/projects/hana-internship/manticore-frontend/v2masmyosys");

let frontend_directory =
    Path::new("/home/hana/projects/hana-internship/manticore-frontend");
```

Update these paths if the Manticore frontend is located somewhere else.

## Usage

Run the program:

```bash
cargo run
```

Enter one Verilog/SystemVerilog path per line:

```text
Enter the path of each Verilog file.
Press Enter on an empty line when finished.
/path/to/top.sv
/path/to/counter.sv
```

After entering all files, press Enter on an empty line.

The files are passed to Yosys together as one design.

The generated output is:

```text
main.masm
```

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
cargo run
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
- Interactive Verilog/SystemVerilog file input
- Automated generation of `main.masm`


## License

No license has been selected for this repository yet.
