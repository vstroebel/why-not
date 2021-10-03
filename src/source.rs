use std::process::exit;

use atty::Stream;

use crate::config::Config;

pub fn get_messages(config: &Config) -> Vec<String> {
    let message = match config.strings.len() {
        0 => {
            // Check if stdin is piped from other app
            if atty::isnt(Stream::Stdin) {
                let mut message = "".to_owned();
                let input = std::io::stdin();

                match input.read_line(&mut message) {
                    Ok(n) => {
                        if n == 1 {
                            get_default(config)
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
                get_default(config)
            }
        }
        _ => config.strings.clone(),
    };

    message
}

fn get_default(config: &Config) -> Vec<String> {
    if config.random {
        vec!["y".to_owned(), "n".to_owned()]
    } else {
        vec!["y".to_owned()]
    }
}
