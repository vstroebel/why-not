#![allow(clippy::collapsible_else_if)]

use std::io::{Error, ErrorKind, Result as IoResult};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

use crate::config::Config;
use crate::source::get_messages;
use crate::writer::Writer;

mod config;
mod source;
mod writer;

pub fn main() {
    let config = Config::load_from_env();

    let w = Writer::new_from_config(&config);

    let res = if config.random {
        print_random_messages(&config, w)
    } else {
        print_message(&config, w)
    };

    if let Err(err) = res {
        eprintln!("{}", err);
    }
}

fn print_message(config: &Config, mut w: Writer) -> IoResult<()> {
    let messages = get_messages(config);

    let message = match messages.first() {
        Some(message) => message.as_str(),
        None => return Err(Error::new(ErrorKind::Other, "Missing messages".to_owned())),
    };

    let message = format!("{}\n", message.to_owned());

    if let Some(max) = config.max_lines {
        for _ in 0..max {
            w.write(&message)?;
        }
    } else {
        loop {
            w.write(&message)?;
        }
    }
    Ok(())
}

fn print_random_messages(config: &Config, mut w: Writer) -> IoResult<()> {
    let messages = get_messages(config);
    let mut rng = thread_rng();

    if let Some(max) = config.max_lines {
        for _ in 0..max {
            print_random_message(&messages, &mut w, &mut rng)?
        }
    } else {
        loop {
            print_random_message(&messages, &mut w, &mut rng)?
        }
    }

    Ok(())
}

fn print_random_message(messages: &[String], w: &mut Writer, rng: &mut ThreadRng) -> IoResult<()> {
    let message = messages.choose(rng).map(|m| m.as_str()).unwrap_or("");
    w.writeln(message)
}
