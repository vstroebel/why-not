use std::str::FromStr;

use clap::{
    crate_description, crate_name, crate_version, value_t, App, Arg, ArgMatches, ErrorKind,
};

const STD_ERR_ARG: &str = "STD_ERR";
const MAX_ARG: &str = "MAX";
const RANDOM_ARG: &str = "RANDOM";
const COLOR_ARG: &str = "COLOR";
const STRING_ARG: &str = "STRING";

pub struct Config {
    pub std_err: bool,
    pub random: bool,
    pub max_lines: Option<usize>,
    pub strings: Vec<String>,
    pub color: Color,
}

#[derive(Copy, Clone)]
pub enum Color {
    None,
    Color(termcolor::Color),
    Random,
}

impl Config {
    pub fn load_from_env() -> Config {
        let app = create_clap_app();
        let matches = app.get_matches();
        Config {
            std_err: use_std_err(&matches),
            random: is_random(&matches),
            max_lines: get_max_lines(&matches),
            strings: get_strings(&matches),
            color: get_color(&matches),
        }
    }
}

fn create_clap_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name(STD_ERR_ARG)
                .short("e")
                .long("stderr")
                .help("Print to stderr"),
        )
        .arg(
            Arg::with_name(MAX_ARG)
                .short("m")
                .long("max")
                .empty_values(false)
                .help("Maximum number of lines to print"),
        )
        .arg(
            Arg::with_name(RANDOM_ARG)
                .short("r")
                .long("random")
                .help("Randomize output strings"),
        )
        .arg(
            Arg::with_name(COLOR_ARG)
                .short("c")
                .long("color")
                .help("Color to use")
                .empty_values(false)
                .possible_values(&[
                    "black", "blue", "green", "red", "cyan", "magenta", "yellow", "white", "random",
                ]),
        )
        .arg(
            Arg::with_name(STRING_ARG)
                .multiple(true)
                .help("String to print. Default: \"y\""),
        )
}

fn use_std_err(matches: &ArgMatches) -> bool {
    matches.is_present(STD_ERR_ARG)
}

fn is_random(matches: &ArgMatches) -> bool {
    matches.is_present(RANDOM_ARG)
}

fn get_max_lines(matches: &ArgMatches) -> Option<usize> {
    match value_t!(matches, MAX_ARG, usize) {
        Ok(max) => Some(max),
        Err(err) => {
            if err.kind == ErrorKind::EmptyValue || err.kind == ErrorKind::ArgumentNotFound {
                None
            } else {
                err.exit()
            }
        }
    }
}

fn get_strings(matches: &ArgMatches) -> Vec<String> {
    matches
        .values_of(STRING_ARG)
        .map(|v| v.map(|v| v.to_owned()).collect())
        .unwrap_or_else(Vec::new)
}

fn get_color(matches: &ArgMatches) -> Color {
    match matches.value_of(COLOR_ARG) {
        Some(value) => match termcolor::Color::from_str(value) {
            Ok(color) => Color::Color(color),
            Err(_) => Color::Random,
        },
        None => Color::None,
    }
}
