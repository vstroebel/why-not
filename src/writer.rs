use std::io::Write;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use termcolor::{BufferedStandardStream, ColorChoice, ColorSpec, WriteColor};

use crate::config::{Color, Config};

pub struct Writer {
    out: BufferedStandardStream,
    color: Color,
}

const COLORS: [termcolor::Color; 8] = [
    termcolor::Color::Black,
    termcolor::Color::White,
    termcolor::Color::Red,
    termcolor::Color::Green,
    termcolor::Color::Blue,
    termcolor::Color::Yellow,
    termcolor::Color::Cyan,
    termcolor::Color::Magenta,
];

impl Writer {
    pub fn new_from_config(config: &Config) -> Writer {
        let out = if config.std_err {
            BufferedStandardStream::stderr(ColorChoice::Auto)
        } else {
            BufferedStandardStream::stdout(ColorChoice::Auto)
        };

        Writer {
            out,
            color: config.color,
        }
    }

    pub fn supports_multiple_messages(&self) -> bool {
        matches!(self.color, Color::None | Color::Color(_))
    }

    pub fn write(&mut self, message: &str) -> std::io::Result<()> {
        match self.color {
            Color::None => self.out.write_all(message.as_bytes()),
            Color::Color(color) => self.write_color(message, ColorSpec::new().set_fg(Some(color))),
            Color::Random => {
                let mut color_spec = ColorSpec::new();
                let mut rng = thread_rng();
                color_spec.set_fg(Some(*COLORS.choose(&mut rng).unwrap()));
                color_spec.set_bold(rng.gen_bool(0.5));
                color_spec.set_italic(rng.gen_bool(0.5));
                color_spec.set_underline(rng.gen_bool(0.5));

                match rng.gen_range(0..=2) {
                    1 => {
                        color_spec.set_dimmed(true);
                    }
                    2 => {
                        color_spec.set_intense(true);
                    }
                    _ => (),
                };

                self.write_color(message, &color_spec)
            }
        }
    }

    fn write_color(&mut self, message: &str, color_spec: &ColorSpec) -> std::io::Result<()> {
        self.out.set_color(color_spec)?;
        self.out.write_all(message.as_bytes())?;
        self.out.reset()
    }

    pub fn writeln(&mut self, message: &str) -> std::io::Result<()> {
        self.write(message)?;
        self.out.write_all("\n".as_bytes())
    }

    pub fn reset(&mut self) -> std::io::Result<()> {
        self.out.reset()?;
        self.out.flush()
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        if let Err(err) = self.reset() {
            eprintln!("{}", err);
        }
    }
}
