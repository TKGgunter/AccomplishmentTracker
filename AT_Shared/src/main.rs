use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

// TODO are doing this wrong?
use accomplishment_tracker_shared::{run, convert};


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input_file_path: PathBuf,

    #[arg(short, long)]
    output_file_path: PathBuf,

    #[arg(long, default_value_t = false)]
    /// converts older toml report formats to the new format.
    /// This is an optional parameter.
    convert_toml: bool
}

fn main() -> Result<(), String>{
    let Args{input_file_path, output_file_path, convert_toml} = Args::parse();
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

    if !convert_toml {
        run(contents, output_file_path);
    } else {
        let old_toml: convert::Report = toml::from_str(&contents).expect("Old report could not be deserialzied.");
        let new_toml = old_toml.write_in_new_format();
        let mut outfile = File::create(output_file_path).expect("Could not create the file.");
        outfile.write(new_toml.as_bytes()).expect("Could not write to file.");
    }

    Ok(())
}
