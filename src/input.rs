use crate::cli::InputArgs;
use std::{
    fs::File,
    io::{Error, Read},
    path::PathBuf,
};

pub struct FileResult {
    pub path: Option<PathBuf>,
    pub bytes: Result<Vec<u8>, Error>,
}

pub fn read_input_as_bytes(args: &mut InputArgs) -> Vec<FileResult> {
    let mut all_results: Vec<FileResult> = Vec::new();

    if !args.files.is_empty() {
        for curr_path in &args.files {
            let curr_file_handle = try_read_file(curr_path).inspect_err(|e| {
                eprintln!(
                    "Encountered error attempting to open a file ({}):\n{}",
                    curr_path.display(),
                    e
                )
            });

            let curr_bytes = match curr_file_handle {
                Ok(h) => generic_reader(h).inspect_err(|e| {
                    eprintln!(
                        "Encountered error when reading a file ({}):\n{}",
                        curr_path.display(),
                        e
                    )
                }),
                Err(e) => Err(e),
            };

            all_results.push(FileResult {
                path: Some(curr_path.to_path_buf()),
                bytes: curr_bytes,
            })
        }
    } else {
        // We're using stdin
        let stdin_bytes = generic_reader(std::io::stdin())
            .inspect_err(|e| eprintln!("Encountered error when reading stdin:\n{}", e));

        all_results.push(FileResult {
            path: None,
            bytes: stdin_bytes,
        });
    }

    all_results
}

fn try_read_file(path: &PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

fn generic_reader<R: Read>(mut reader: R) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
