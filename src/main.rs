#![allow(clippy::collapsible_else_if)]

mod config;
mod source;

use crate::config::Config;
use crate::source::get_messages;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use std::io::{Error, ErrorKind, Result as IoResult, Write};

const BUFF_SIZE: usize = 8192;

pub fn main() {
    let config = Config::load_from_env();

    let res = if config.random {
        print_random_messages(&config)
    } else {
        print_message(&config)
    };

    if let Err(err) = res {
        eprintln!("{}", err);
    }
}

fn print_message(config: &Config) -> IoResult<()> {
    let messages = get_messages(config);

    let message = match messages.first() {
        Some(message) => message.as_str(),
        None => return Err(Error::new(ErrorKind::Other, "Missing messages".to_owned())),
    };

    if let Some(max) = config.max_lines {
        if config.std_err {
            print_max(std::io::stderr(), message, max)
        } else {
            print_max(std::io::stdout(), message, max)
        }
    } else {
        if config.std_err {
            print_infinitely(std::io::stderr(), message)
        } else {
            print_infinitely(std::io::stdout(), message)
        }
    }
}

fn print_random_messages(config: &Config) -> IoResult<()> {
    let messages = get_messages(config);
    let mut rng = thread_rng();

    if let Some(max) = config.max_lines {
        for _ in 0..max {
            print_random_message(&messages, config.std_err, &mut rng)?
        }
    } else {
        loop {
            print_random_message(&messages, config.std_err, &mut rng)?
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
