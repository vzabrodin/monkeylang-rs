use crate::lexer::Lexer;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

const PROMPT: &str = ">> ";

pub fn start(read: &mut impl Read, write: &mut impl Write) {
    let mut reader = BufReader::new(read);
    let mut writer = BufWriter::new(write);

    loop {
        let _ = writer.write(PROMPT.as_bytes());
        let _ = writer.flush();

        let mut input = String::new();
        let _ = reader.read_line(&mut input);

        let lexer = Lexer::new(&input);
        for token in lexer {
            let _ = writer.write_fmt(format_args!("{:?} ", token));
        }

        let _ = writer.write(b"\n");
        let _ = writer.flush();
    }
}
