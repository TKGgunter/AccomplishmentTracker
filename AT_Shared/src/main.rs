use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

// TODO we are doing this wrong.
use accomplishment_tracker_shared::{AccomplishmentData, TomlAccomplishmentData, serialize_to_file, run};


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input_file_path: PathBuf,

    #[arg(short, long)]
    output_file_path: PathBuf
}

fn main() -> Result<(), String>{
    let Args{input_file_path, output_file_path} = Args::parse();
    // TODO Check the path for file.
    let mut file = match File::open(input_file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("{:?}", e);
            return Err("Could not open file.".to_string());
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file to string.");

    run(contents, output_file_path);

    Ok(())
}
