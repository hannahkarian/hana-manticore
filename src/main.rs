use std::env;
use std::error::Error;
use std::process::{self, Command};
// Path: borrowed path, for fixed paths
// PathBuf: owned path, for paths entered by the user
use std::path::{Path, PathBuf};

// returns the name or path of the Manticore executable
fn frontend_bin() -> String {
    // use the path from the environment variable if it is set,
    // otherwise use only the name, so Command::new() searches the PATH for it
    env::var("MANTICORE_YOSYS_FRONTEND").unwrap_or_else(|_| String::from("v2masmyosys"))
}

// builds the commands that are passed to the frontend with -p
fn build_yosys_commands(verilog_files: &[PathBuf]) -> String {
    // start building the Yosys command
    let mut yosys_commands = String::from("read_verilog -sv -masm");

    // add all Verilog file paths to the read_verilog command
    for file_path in verilog_files {
        yosys_commands.push(' '); // add a space after -masm
        yosys_commands.push_str(&format!("\"{}\"", file_path.display())); //display() converts the path into printable text
    }

    // add the rest of the pipeline:
    //* Checks whether the module hierarchy is valid.
    //* Automatically finds the top module.
    //* Converts procedural Verilog code, such as always blocks, into netlist cells.
    //* Optimizes and cleans the netlist.
    //* Checks the design and treats detected problems as errors.
    //* Processes memory-related structures.
    //* Prepares the design for Manticore and removes module hierarchy with flatten.
    //* Runs Manticore-specific transformations and checks.
    //* Prints statistics about the resulting design.
    //* Converts the processed netlist into Manticore assembly and writes it to main.masm.
    yosys_commands.push_str(
        "; hierarchy -check \
        ; hierarchy -auto-top \
        ; proc \
        ; opt -purge \
        ; wreduce -memx \
        ; opt_reduce \
        ; opt_clean \
        ; check -assert \
        ; memory_collect \
        ; memory_unpack \
        ; memory_dff \
        ; manticore_init \
        ; flatten \
        ; manticore_meminit \
        ; opt -purge \
        ; manticore_dff \
        ; manticore_opt_replicate \
        ; manticore_subword \
        ; manticore_check \
        ; stat \
        ; manticore_writer main.masm",
    );

    yosys_commands
}

//Result: Ok(PathBuf), Err(Box<dyn Error>)
// only runs the frontend, the files are read in main
fn run_manticore_frontend(
    verilog_files: &[PathBuf],
    working_dir: &Path,
) -> Result<PathBuf, Box<dyn Error>> {
    let frontend_binary = frontend_bin();
    let yosys_commands = build_yosys_commands(verilog_files);

    // run v2masmyosys -p "read_verilog ... ; manticore_writer main.masm"
    // status() shows the output from Yosys and Manticore directly in the terminal
    let result = Command::new(&frontend_binary)
        .current_dir(working_dir)
        .arg("-p")
        .arg(&yosys_commands)
        .status()
        .map_err(|error| format!("could not start {}: {}", frontend_binary, error))?; // explains which program could not be started

    if !result.success() {
        return Err("The Manticore frontend failed.".into());
    }

    // combine the working directory and filename as one path
    let masm_file = working_dir.join("main.masm");

    if !masm_file.is_file() {
        return Err("main.masm was not created.".into());
    }

    // return the path of main.masm (PathBuf)
    Ok(masm_file)
}

// reads the command line arguments, checks the files and runs the frontend
fn run() -> Result<PathBuf, Box<dyn Error>> {
    // new vector stores all Verilog file paths given by the user
    // mutable because we will add paths
    let mut verilog_files: Vec<PathBuf> = Vec::new();

    // the directory where the frontend runs and main.masm is created
    // None means the user did not choose one
    let mut working_dir: Option<PathBuf> = None;

    // skip(1) skips the first argument, which is the name of the program
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--working-dir" {
            // the directory is the next argument
            let dir = args.next().ok_or("--working-dir needs a directory")?;
            working_dir = Some(PathBuf::from(dir));
            continue;
        }

        // converts the text path into a PathBuf
        let file_path = PathBuf::from(arg);

        // check if the entered path is a file, not a directory
        if !file_path.is_file() {
            return Err(format!("File does not exist: {}", file_path.display()).into());
        }

        // make the path absolute, so it still works when the frontend runs in another directory
        verilog_files.push(file_path.canonicalize()?);
    }

    if verilog_files.is_empty() {
        return Err("No Verilog files were provided.\nUsage: hana-manticore [--working-dir DIR] FILE.v [FILE.v ...]".into()); // converts the text error into the function’s required error type
    }

    // if no directory was given, use the directory the program was started from
    let working_dir = match working_dir {
        Some(dir) => dir,
        None => env::current_dir()?,
    };

    run_manticore_frontend(&verilog_files, &working_dir)
}

fn main() {
    match run() {
        Ok(masm_file) => {
            println!("Created: {}", masm_file.display());
            println!("You can now parse this file:");
            println!("{}", masm_file.display());
        }

        Err(error) => {
            // eprintln! prints errors to stderr instead of stdout
            eprintln!("Error: {}", error);
            // exit code 1 tells the terminal (and scripts) that something went wrong
            process::exit(1);
        }
    }
}
