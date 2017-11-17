//!
//! cargo-download
//!

             extern crate ansi_term;
#[macro_use] extern crate clap;
             extern crate conv;
#[macro_use] extern crate derive_error;
             extern crate exitcode;
             extern crate isatty;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
             extern crate slog_envlogger;
             extern crate slog_stdlog;
             extern crate slog_stream;
             extern crate time;

// `slog` must precede `log` in declarations here, because we want to simultaneously:
// * use the standard `log` macros
// * be able to initialize the slog logger using slog macros like o!()
#[macro_use] extern crate slog;
#[macro_use] extern crate log;


mod args;
mod logging;


use std::io::{self, Write};
use std::process::exit;

use log::LogLevel::*;

use args::ArgsError;


lazy_static! {
    /// Application / package name, as filled out by Cargo.
    static ref NAME: &'static str = option_env!("CARGO_PKG_NAME")
        .unwrap_or("cargo-download");

    /// Application version, as filled out by Cargo.
    static ref VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
}


fn main() {
    let opts = args::parse().unwrap_or_else(|e| {
        print_args_error(e).unwrap();
        exit(exitcode::USAGE);
    });

    logging::init(opts.verbosity).unwrap();
    log_signature();
}

// Print an error that may occur while parsing arguments.
fn print_args_error(e: ArgsError) -> io::Result<()> {
    match e {
        ArgsError::Parse(ref e) =>
            // In case of generic parse error,
            // message provided by the clap library will be the usage string.
            writeln!(&mut io::stderr(), "{}", e.message),
    }
}

/// Log the program name, version, and other metadata.
#[inline]
fn log_signature() {
    if log_enabled!(Info) {
        let version = VERSION.map(|v| format!("v{}", v))
            .unwrap_or_else(|| "<UNKNOWN VERSION>".into());
        info!("{} {}", *NAME, version);
    }
}
