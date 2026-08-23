use clap::Parser;
use wc_rust::{InputArgs, resolve_defaults};

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
