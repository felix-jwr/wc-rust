use clap::{Args, Parser};
use std::path::PathBuf;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_flags_defaults_to_lwc() {
        let mut args = InputArgs::parse_from(["wc"]);
        resolve_defaults(&mut args);
        assert!(args.l);
        assert!(args.w);
        assert!(args.output_mode.c);
        assert!(!args.output_mode.m);
    }

    #[test]
    fn explicit_l_does_not_trigger_defaults() {
        let mut args = InputArgs::parse_from(["wc", "-l"]);
        resolve_defaults(&mut args);
        assert!(args.l);
        assert!(!args.w);
        assert!(!args.output_mode.c);
    }

    #[test]
    fn c_and_m_together_is_rejected() {
        let result = InputArgs::try_parse_from(["wc", "-c", "-m"]);
        assert!(result.is_err());
    }

    #[test]
    fn c_alone_is_accepted() {
        let result = InputArgs::try_parse_from(["wc", "-c"]);
        assert!(result.is_ok());
    }

    #[test]
    fn positional_files_are_collected() {
        let args = InputArgs::parse_from(["wc", "a.txt", "b.txt"]);
        assert_eq!(args.files.len(), 2);
    }

    #[test]
    fn no_files_gives_empty_vec() {
        let args = InputArgs::parse_from(["wc"]);
        assert!(args.files.is_empty());
    }
}
