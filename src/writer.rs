use std::io::{BufWriter, Write};

pub struct Writer {
    out: BufWriter<Box<dyn Write>>,
}

impl Writer {
    pub fn new<W: 'static + Write>(w: W) -> Writer {
        Writer {
            out: BufWriter::new(Box::new(w)),
        }
    }

    pub fn write(&mut self, message: &str) -> std::io::Result<()> {
        self.out.write_all(message.as_bytes())
    }
}
