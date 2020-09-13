use std::io;
use std::io::BufRead;
use crate::{lexer, token};

pub fn start<R : io::Read, W : io::Write>(mut io_read : R, mut io_write : W) {
    let mut input = String::new();
    let mut buf = std::io::BufReader::new(io_read);

    io_write.write(">> ".as_bytes()).unwrap();
    io_write.flush().unwrap();

    buf.read_line(&mut input).unwrap();

    let mut lex = lexer::new(input);

    loop {
        let token = lex.next_token();

        if token.token_type == token::EOF {
            break;
        }

        println!("{:?}", token);
    }
}
