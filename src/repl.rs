use crate::{lexer, token};
use std::io;
use std::io::BufRead;

pub fn start<R: io::Read, W: io::Write>(io_read: R, io_write: &mut W) {
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

        io_write.write(format!("{:?}\n", token).as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start() {
        let read = "5 * 5 = 10;".as_bytes();
        let mut output: Vec<u8> = vec![];

        start(read, &mut output);

        let expected = r#">> Token { token_type: "INT", literal: "5" }
Token { token_type: "*", literal: "*" }
Token { token_type: "INT", literal: "5" }
Token { token_type: "=", literal: "=" }
Token { token_type: "INT", literal: "10" }
Token { token_type: ";", literal: ";" }
"#;

        assert_eq!(expected, std::str::from_utf8(&output).unwrap());
    }
}
