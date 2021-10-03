#![allow(clippy::collapsible_else_if)]

use std::env;
use clap::{App, Arg, ArgMatches, crate_name, crate_version, crate_description, value_t, ErrorKind};
use std::io::{Write, Result as IoResult};
use std::process::exit;
use atty::Stream;
use rand::{thread_rng, seq::SliceRandom};
use rand::rngs::ThreadRng;

const BUFF_SIZE: usize = 8192;

const STD_ERR_ARG: &str = "STD_ERR";
const MAX_ARG: &str = "MAX";
const RANDOM_ARG: &str = "RANDOM";
const STRING_ARG: &str = "STRING";

pub fn main() {
    let app = create_clap_app();

    let matches = app.get_matches();

    let res = if matches.is_present(RANDOM_ARG) {
        print_random_messages(&matches)
    } else {
        print_message(&matches)
    };

    if let Err(err) = res {
        eprintln!("{}", err);
    }
}

fn print_message(matches: &ArgMatches) -> IoResult<()> {
    let message = get_message(matches);
    let max = get_max_lines(matches);
    let stderr = matches.is_present(STD_ERR_ARG);

    if let Some(max) = max {
        if stderr {
            print_max(std::io::stderr(), &message, max)
        } else {
            print_max(std::io::stdout(), &message, max)
        }
    } else {
        if stderr {
            print_infinitely(std::io::stderr(), &message)
        } else {
            print_infinitely(std::io::stdout(), &message)
        }
    }
}

fn print_random_messages(matches: &ArgMatches) -> IoResult<()> {
    let messages = get_random_messages(matches);
    let max = get_max_lines(matches);
    let stderr = matches.is_present(STD_ERR_ARG);

    let mut rng = thread_rng();

    if let Some(max) = max {
        for _ in 0..max {
            print_random_message(&messages, stderr, &mut rng)?
        }
    } else {
        loop {
            print_random_message(&messages, stderr, &mut rng)?
        }
    }

    Ok(())
}

fn print_random_message(messages: &[String], stderr: bool, rng: &mut ThreadRng) -> IoResult<()> {
    let message = messages.choose(rng).map(|m| m.as_str()).unwrap_or("");
    if stderr {
        writeln!(std::io::stderr(), "{}", message)
    } else {
        writeln!(std::io::stdout(), "{}", message)
    }
}

fn get_max_lines(matches: &ArgMatches) -> Option<usize> {
    match value_t!(matches, MAX_ARG, usize) {
        Ok(max) => Some(max),
        Err(err) => if err.kind == ErrorKind::EmptyValue || err.kind == ErrorKind::ArgumentNotFound {
            None
        } else {
            err.exit()
        },
    }
}

fn print_max<W: Write>(mut w: W, message: &str, max: usize) -> IoResult<()> {
    let (buffer, buff_count) = get_buffer(message, Some(max));

    for _ in 0..max / buff_count {
        writeln!(w, "{}", buffer)?;
    }

    for _ in 0..max % buff_count {
        writeln!(w, "{}", message)?;
    }

    Ok(())
}

fn print_infinitely<W: Write>(mut w: W, message: &str) -> IoResult<()> {
    let (buffer, _) = get_buffer(message, None);
    loop {
        w.write_all(buffer.as_bytes())?;
    }
}

fn get_buffer(message: &str, max: Option<usize>) -> (String, usize) {
    let mut total = (BUFF_SIZE / (message.len() + 1)).max(1);

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
        None => {
            // Check if stdin is piped from other app
            if atty::isnt(Stream::Stdin) {
                let mut message = "".to_owned();
                let input = std::io::stdin();

                match input.read_line(&mut message) {
                    Ok(n) => {
                        if n == 1 {
                            message.push('y');
                        } else {
                            // Remove newline at end of line
                            message.pop();
                        }
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(1);
                    }
                }

                message
            } else {
                "y".to_owned()
            }
        }
    };
    message
}

fn get_random_messages(matches: &ArgMatches) -> Vec<String> {
    let message = match matches.values_of(STRING_ARG) {
        Some(values) => values.map(|s| s.to_owned()).collect(),
        None => {
            // Check if stdin is piped from other app
            if atty::isnt(Stream::Stdin) {
                let mut message = "".to_owned();
                let input = std::io::stdin();

                match input.read_line(&mut message) {
                    Ok(n) => {
                        if n == 1 {
                            vec!["y".to_owned(), "n".to_owned()]
                        } else {
                            // Remove newline at end of line
                            message.pop();
                            message.split(' ').map(|s| s.to_owned()).collect()
                        }
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(1);
                    }
                }
            } else {
                vec!["y".to_owned(), "n".to_owned()]
            }
        }
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
        .arg(Arg::with_name(RANDOM_ARG)
            .short("r")
            .long("random")
            .help("Randomize output strings"))
        .arg(Arg::with_name(STRING_ARG)
            .multiple(true)
            .help("String to print. Default: \"y\""))
}
