use std::error::Error;
use std::io;
use std::process::Command;
// Path: borrowed path, for fixed paths
// PathBuf: owned path, for paths entered by the user
use std::path::{Path, PathBuf};

//Result: Ok(PathBuf), Err(Box<dyn Error>)
fn run_manticore_frontend() -> Result<PathBuf, Box<dyn Error>> {
    // location of the Manticore executable
    let frontend_binary =
        Path::new("/home/hana/projects/hana-internship/manticore-frontend/v2masmyosys");

    // directory where the executable runs and main.masm is created
    let frontend_directory = Path::new("/home/hana/projects/hana-internship/manticore-frontend");

    // new vector stores all Verilog file paths entered by the user
    // mutable because we will add paths
    let mut verilog_files: Vec<PathBuf> = Vec::new();

    println!("Enter the path of each Verilog file.");
    println!("Press Enter on an empty line when finished.");

    loop {
        let mut input = String::new();

        // read one line and place it into input
        io::stdin().read_line(&mut input)?;

        // remove the newline and spaces
        let input = input.trim();

        // when the user presses Enter on an empty line, break
        if input.is_empty() {
            break;
        }

        // converts the text path into a PathBuf
        let file_path = PathBuf::from(input);

        // check if the entered path is a file, not a directory
        if !file_path.is_file() {
            println!("File does not exist!");
            continue;
        }

        // add the valid file path to vector
        verilog_files.push(file_path);
    }

    if verilog_files.is_empty() {
        return Err("No Verilog files were provided.".into()); // converts the text error into the function’s required error type
    }

    // start building the Yosys command
    let mut yosys_commands = String::from("read_verilog -sv -masm");

    // add all Verilog file paths to the read_verilog command
    for file_path in &verilog_files {
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

    // run v2masmyosys -p "read_verilog ... ; manticore_writer main.masm"
    let result = Command::new(frontend_binary)
        .current_dir(frontend_directory)
        .arg("-p")
        .arg(&yosys_commands)
        .output()?;

    // show the output from Yosys and Manticore
    // Command::output() returns bytes, so we convert them into text with String::from_utf8_lossy()
    println!("{}", String::from_utf8_lossy(&result.stdout));

    if !result.status.success() {
        println!("{}", String::from_utf8_lossy(&result.stderr));
        return Err("The Manticore frontend failed.".into());
    }

    // combine the frontend directory and filename as one path
    let masm_file = frontend_directory.join("main.masm");

    if !masm_file.is_file() {
        return Err("main.masm was not created.".into());
    }

    println!("Created: {}", masm_file.display());

    // return the path of main.masm (PathBuf))
    Ok(masm_file)
}

fn main() {
    match run_manticore_frontend() {
        Ok(masm_file) => {
            println!("You can now parse this file:");
            println!("{}", masm_file.display());
        }

        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
