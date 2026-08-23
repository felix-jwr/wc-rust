use clap::{Args, Parser};
use std::{
    fs::File,
    io::{Error, Read},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author = "Felix", version = "0.1.0")]
#[command(about = "Implementation of the word counter utility 'wc' in Rust")]
pub struct InputArgs {
    #[command(flatten)]
    pub output_mode: OutputMode,

    #[arg(short, long)]
    pub w: bool,

    #[arg(short, long)]
    pub l: bool,

    #[arg()]
    pub files: Vec<PathBuf>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct OutputMode {
    #[arg(short, long)]
    pub c: bool,

    #[arg(short, long)]
    pub m: bool,
}

pub fn resolve_defaults(args: &mut InputArgs) {
    if !args.output_mode.c && !args.output_mode.m && !args.l && !args.w {
        args.l = true;
        args.w = true;
        args.output_mode.c = true;
    }
}

pub fn read_input_as_bytes(args: &mut InputArgs) -> Vec<Vec<u8>> {
    let mut all_bytes: Vec<Vec<u8>> = Vec::new();

    if !args.files.is_empty() {
        for file_path in &args.files {
            let file_handle = try_read_file(file_path);

            match file_handle {
                // handle ok, read file bytes
                Ok(h) => {
                    let curr_bytes = generic_reader(h);
                    match curr_bytes {
                        Ok(b) => all_bytes.push(b), // add curr file contents to 2d vec
                        Err(e) => eprintln!(
                            "Encountered error when reading a file ({}):\n{}",
                            file_path.display(),
                            e
                        ),
                    }
                }
                Err(e) => eprintln!(
                    "Encountered error when reading a file ({}):\n{}",
                    file_path.display(),
                    e
                ),
            }
        }
    } else {
        let curr_bytes = generic_reader(std::io::stdin());

        match curr_bytes {
            Ok(b) => all_bytes.push(b),
            Err(e) => eprintln!("Encountered error when reading a stdin:\n{}", e),
        }
    }

    all_bytes
}

pub fn try_read_file(path: &PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

pub fn generic_reader<R: Read>(mut reader: R) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
