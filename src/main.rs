#![allow(clippy::collapsible_else_if)]

use std::env;
use clap::{App, Arg, ArgMatches, crate_name, crate_version, crate_description, value_t, ErrorKind};
use std::io::Write;

const BUFF_SIZE: usize = 8192;

const STD_ERR_ARG: &str = "STD_ERR";
const MAX_ARG: &str = "MAX";
const STRING_ARG: &str = "STRING";

pub fn main() {
    let app = create_clap_app();

    let matches = app.get_matches();

    let message = get_message(&matches);

    let max = match value_t!(matches, MAX_ARG, usize) {
        Ok(max) => Some(max),
        Err(err) => if err.kind == ErrorKind::EmptyValue || err.kind == ErrorKind::ArgumentNotFound {
            None
        } else {
            err.exit()
        },
    };

    let stderr = matches.is_present(STD_ERR_ARG);

    if let Some(max) = max {
        if stderr {
            print_max(std::io::stderr(), &message, max);
        } else {
            print_max(std::io::stdout(), &message, max);
        }
    } else {
        if stderr {
            print_infinitely(std::io::stderr(), &message);
        } else {
            print_infinitely(std::io::stdout(), &message);
        }
    }
}

fn print_max<W: Write>(mut w: W, message: &str, max: usize) {
    let (buffer, buff_count) = get_buffer(message, Some(max));

    for _ in 0..max / buff_count {
        writeln!(w, "{}", buffer);
    }

    for _ in 0..max % buff_count {
        writeln!(w, "{}", message);
    }
}

fn print_infinitely<W: Write>(mut w: W, message: &str) {
    let (buffer, _) = get_buffer(message, None);
    loop {
        w.write_all(buffer.as_bytes());
    }
}

fn get_buffer(message: &str, max: Option<usize>) -> (String, usize) {
    let mut total = BUFF_SIZE / (message.len() + 1);

    if let Some(max) = max {
        if max == 0 {
            return ("".to_owned(), 0);
        }
        if total > max {
            total = max
        }
    }

    let mut buffer = String::with_capacity(BUFF_SIZE);

    buffer.push_str(message);
    buffer.push('\n');

    for _ in 1..total {
        buffer.push_str(message);
        buffer.push('\n');
    }

    (buffer, total)
}

fn get_message(matches: &ArgMatches) -> String {
    let message = match matches.values_of(STRING_ARG) {
        Some(values) => {
            values.fold("".to_owned(), |mut message, v| {
                if !message.is_empty() {
                    message.push(' ');
                }
                message.push_str(v);
                message
            })
        }
        None => "y".to_owned()
    };
    message
}

fn create_clap_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name(STD_ERR_ARG)
            .short("e")
            .long("stderr")
            .help("Print to stderr"))
        .arg(Arg::with_name(MAX_ARG)
            .short("m")
            .long("max")
            .empty_values(false)
            .help("Maximum number of lines to print"))
        .arg(Arg::with_name(STRING_ARG)
            .default_value("y")
            .multiple(true))
}
