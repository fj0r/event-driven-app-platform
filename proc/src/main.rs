//! Parse a Rust source file into a `syn::File` and print out a debug
//! representation of the syntax tree.
//!
//! Use the following command from this directory to test this program by
//! running it on its own source code:
//!
//!     cargo run -- src/main.rs
//!
//! The output will begin with:
//!
//!     File {
//!         shebang: None,
//!         attrs: [
//!             Attribute {
//!                 pound_token: Pound,
//!                 style: AttrStyle::Inner(
//!         ...
//!     }

use colored::Colorize;
use serde_json::Value;
use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
use std::fmt::{self, Display};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

enum Error {
    IncorrectUsage,
    ReadFile(io::Error),
    ParseFile {
        error: syn::Error,
        filepath: PathBuf,
        source_code: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IncorrectUsage => write!(f, "Usage: dump-syntax path/to/filename.rs"),
            Error::ReadFile(error) => write!(f, "Unable to read file: {}", error),
            Error::ParseFile {
                error,
                filepath,
                source_code,
            } => render_location(f, error, filepath, source_code),
        }
    }
}

fn main() {
    if let Err(error) = try_main() {
        let _ = writeln!(io::stderr(), "{}", error);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    let mut args = env::args_os();
    let _ = args.next(); // executable name

    let filepath = match (args.next(), args.next()) {
        (Some(arg), None) => PathBuf::from(arg),
        _ => return Err(Error::IncorrectUsage),
    };

    let code = fs::read_to_string(&filepath).map_err(Error::ReadFile)?;
    let syntax = syn::parse_file(&code).map_err({
        |error| Error::ParseFile {
            error,
            filepath,
            source_code: code,
        }
    })?;

    println!("{:#?}", syntax);

    Ok(())
}

// Render a rustc-style error message, including colors.
//
//     error: Syn unable to parse file
//       --> main.rs:40:17
//        |
//     40 |     fn fmt(&self formatter: &mut fmt::Formatter) -> fmt::Result {
//        |                  ^^^^^^^^^ expected `,`
//
fn render_location(
    formatter: &mut fmt::Formatter,
    err: &syn::Error,
    filepath: &Path,
    code: &str,
) -> fmt::Result {
    let start = err.span().start();
    let mut end = err.span().end();

    let code_line = match start.line.checked_sub(1).and_then(|n| code.lines().nth(n)) {
        Some(line) => line,
        None => return render_fallback(formatter, err),
    };

    if end.line > start.line {
        end.line = start.line;
        end.column = code_line.len();
    }

    let filename = filepath
        .file_name()
        .map(OsStr::to_string_lossy)
        .unwrap_or(Cow::Borrowed("main.rs"));

    write!(
        formatter,
        "\n\
         {error}{header}\n\
         {indent}{arrow} {filename}:{linenum}:{colnum}\n\
         {indent} {pipe}\n\
         {label} {pipe} {code}\n\
         {indent} {pipe} {offset}{underline} {message}\n\
         ",
        error = "error".red().bold(),
        header = ": Syn unable to parse file".bold(),
        indent = " ".repeat(start.line.to_string().len()),
        arrow = "-->".blue().bold(),
        filename = filename,
        linenum = start.line,
        colnum = start.column,
        pipe = "|".blue().bold(),
        label = start.line.to_string().blue().bold(),
        code = code_line.trim_end(),
        offset = " ".repeat(start.column),
        underline = "^"
            .repeat(end.column.saturating_sub(start.column).max(1))
            .red()
            .bold(),
        message = err.to_string().red(),
    )
}

fn render_fallback(formatter: &mut fmt::Formatter, err: &syn::Error) -> fmt::Result {
    write!(formatter, "Unable to parse file: {}", err)
}
