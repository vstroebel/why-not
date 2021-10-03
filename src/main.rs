use std::env;
use clap::{App, Arg, ArgMatches, crate_name, crate_version, crate_description, value_t, ErrorKind};

const BUFF_SIZE: usize = 8192;

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

    if let Some(max) = max {
        for _ in 0..max {
            println!("{}", message)
        }
    } else {
        let buffer = get_buffer(&message);

        loop {
            print!("{}", buffer)
        }
    }
}

fn get_buffer(message: &str) -> String {
    let mut buffer = String::with_capacity(BUFF_SIZE);

    buffer.push_str(message);
    buffer.push('\n');

    for _ in 1..(BUFF_SIZE / (message.len() + 1)) {
        buffer.push_str(message);
        buffer.push('\n');
    }
    buffer
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
        .arg(Arg::with_name(MAX_ARG)
            .short("m")
            .long("max")
            .empty_values(false)
            .help("Maximum number of lines to print"))
        .arg(Arg::with_name(STRING_ARG)
            .default_value("y")
            .multiple(true))
}
