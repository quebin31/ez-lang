use super::token::{self, Token};
use std::error::Error;
use std::fmt::Debug;
use std::io::Read;
use std::iter::{Filter, Iterator};

#[derive(Debug, Clone)]
pub struct Lexer<R>
where
    R: Read + Debug + Clone,
{
    inner: R,
    buf: Vec<char>,
    aux: Vec<char>,
    cursor: usize,
}

pub type FilteredLexer<R> = Filter<Lexer<R>, Box<dyn FnMut(&Token) -> bool>>;

impl<R> Lexer<R>
where
    R: Read + Debug + Clone,
{
    const BUF_SIZE: usize = 8192;
    const AUX_SIZE: usize = 4096;

    pub fn new(inner: R) -> Self {
        Lexer {
            inner,
            buf: Vec::new(),
            aux: Vec::new(),
            cursor: 0,
        }
    }

    pub fn without_whitespaces(inner: R) -> FilteredLexer<R> {
        Self::new(inner).filter(Box::new(|t: &Token| !t.is_whitespace()))
    }
}

impl<R> Lexer<R>
where
    R: Read + Debug + Clone,
{
    fn fill_buf(&mut self) -> Result<usize, Box<dyn Error>> {
        let mut bytes = vec![0; Self::BUF_SIZE - self.aux.len()];
        let bytes_read = self.inner.read(&mut bytes)?;
        if bytes_read == 0 {
            return Ok(bytes_read);
        }

        let mut contents = String::from_utf8(bytes)?.chars().collect();

        self.aux.append(&mut contents);
        self.buf.clear();
        self.buf.append(&mut self.aux);
        self.buf.truncate(bytes_read);
        self.cursor = 0;

        Ok(bytes_read)
    }

    fn fill_aux(&mut self) -> Result<usize, Box<dyn Error>> {
        let mut bytes = vec![0; Self::AUX_SIZE];
        let bytes_read = self.inner.read(&mut bytes)?;
        if bytes_read == 0 {
            return Ok(bytes_read);
        }

        self.aux = String::from_utf8(bytes)?.chars().collect();
        self.aux.truncate(bytes_read);

        Ok(bytes_read)
    }

    fn peek_ahead(&mut self, offset: usize) -> Option<char> {
        let lookahead_cursor = self.cursor + offset;

        if !self.aux.is_empty() {
            let lookahead_cursor_aux = lookahead_cursor - (self.buf.len() - 1) - 1;
            return Some(self.aux[lookahead_cursor_aux]);
        }

        if lookahead_cursor >= self.buf.len() {
            if self.fill_aux().expect("Failed to read!") == 0 {
                None
            } else {
                let lookahead_cursor_aux = lookahead_cursor - (self.buf.len() - 1) - 1;
                Some(self.aux[lookahead_cursor_aux])
            }
        } else {
            Some(self.buf[lookahead_cursor])
        }
    }

    fn peek_curr(&mut self) -> Option<char> {
        if self.cursor >= self.buf.len() && self.fill_buf().expect("Failed to read!") == 0 {
            None
        } else {
            Some(self.buf[self.cursor])
        }
    }

    fn consume_curr(&mut self) {
        self.cursor += 1;
    }

    fn match_number(&mut self) -> Option<Token> {
        let mut number = "".to_owned();

        let mut is_float = false;
        'integer: while let Some(chr) = self.peek_curr() {
            match chr {
                digit if digit.is_ascii_digit() => {
                    number.push(digit);
                    self.consume_curr();
                }

                dot @ '.' => {
                    is_float = true;

                    number.push(dot);
                    self.consume_curr();

                    while let Some(chr) = self.peek_curr() {
                        if chr.is_ascii_digit() {
                            number.push(chr);
                            self.consume_curr();
                        } else {
                            break 'integer;
                        }
                    }
                }

                _ => break 'integer,
            }
        }

        if is_float {
            Some(Token::Float(number.parse().unwrap()))
        } else {
            Some(Token::Integer(number.parse().unwrap()))
        }
    }

    fn match_word(&mut self) -> Option<Token> {
        let mut lexeme = "".to_owned();

        while let Some(chr) = self.peek_curr() {
            match chr {
                valid if valid.is_ascii_alphanumeric() || valid == '_' => {
                    lexeme.push(valid);
                    self.consume_curr();
                }
                _ => break,
            }
        }

        if token::is_reserved_word(&lexeme) {
            Some(Token::ReservedWord(lexeme))
        } else {
            Some(Token::Identifier(lexeme))
        }
    }

    fn match_op(&mut self, op: &str) -> Option<Token> {
        self.consume_curr();
        let possible_output = match op {
            "+" => (Token::Plus, Token::PlusAssign),
            "-" => (Token::Minus, Token::MinusAssign),
            "*" => (Token::Asterisk, Token::AsteriskAssign),
            "/" => (Token::Divide, Token::DivideAssign),
            "%" => (Token::Percetange, Token::PercetangeAssign),
            "&" => (Token::Ampersand, Token::AmpersandAssign),
            "|" => (Token::VerticalBar, Token::VerticalBarAssign),
            "~" => (Token::Tilde, Token::TildeAssign),
            "=" => (Token::Assign, Token::Equal),
            "<" => (Token::LessThan, Token::LessEqual),
            ">" => (Token::GreaterThan, Token::GreaterEqual),
            "!" => (Token::ExclamationMark, Token::NotEqual),
            "&&" => (Token::DoubleAmpersand, Token::DoubleAmpersandAssign),
            "||" => (Token::DoubleVerticalBar, Token::DoubleVerticalBarAssign),
            what => unreachable!("Unreachable with {}", what),
        };

        match self.peek_curr() {
            Some('=') => {
                self.consume_curr();
                Some(possible_output.1)
            }

            _ => Some(possible_output.0),
        }
    }
}

impl<R> Iterator for Lexer<R>
where
    R: Read + Debug + Clone,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peek_curr()? {
            chr if chr.is_ascii_digit() => self.match_number(),
            chr if chr.is_ascii_alphabetic() || chr == '_' => self.match_word(),
            '-' => match self.peek_ahead(1) {
                Some('>') => {
                    self.consume_curr();
                    self.consume_curr();
                    Some(Token::SingleArrow)
                }

                _ => self.match_op("-"),
            },

            ',' => {
                self.consume_curr();
                Some(Token::Coma)
            }
            ';' => {
                self.consume_curr();
                Some(Token::SemiColon)
            }

            ':' => {
                self.consume_curr();
                match self.peek_curr() {
                    Some(':') => {
                        self.consume_curr();
                        Some(Token::ColonPath)
                    }

                    _ => Some(Token::Colon),
                }
            }

            '.' => {
                self.consume_curr();
                match (self.peek_curr(), self.peek_ahead(1)) {
                    (Some('.'), Some('.')) => {
                        self.consume_curr();
                        self.consume_curr();
                        Some(Token::TripleDots)
                    }

                    (Some('.'), _) => {
                        self.consume_curr();
                        Some(Token::DoubleDots)
                    }

                    (_, _) => Some(Token::Dot),
                }
            }

            '*' => self.match_op("*"),
            '+' => self.match_op("+"),
            '/' => self.match_op("/"),
            '~' => self.match_op("~"),
            '!' => self.match_op("!"),
            '=' => match self.peek_ahead(1) {
                Some('>') => {
                    self.consume_curr();
                    self.consume_curr();
                    Some(Token::DoubleArrow)
                }

                _ => self.match_op("="),
            },
            '<' => self.match_op("<"),
            '>' => self.match_op(">"),
            '%' => self.match_op("%"),

            '&' => match self.peek_ahead(1) {
                Some('&') => {
                    self.consume_curr();
                    self.match_op("&&")
                }

                _ => self.match_op("&"),
            },

            '|' => match self.peek_ahead(1) {
                Some('|') => {
                    self.consume_curr();
                    self.match_op("||")
                }
                _ => self.match_op("|"),
            },

            '{' => {
                self.consume_curr();
                Some(Token::LeftBrace)
            }
            '}' => {
                self.consume_curr();
                Some(Token::RightBrace)
            }
            '(' => {
                self.consume_curr();
                Some(Token::LeftParenthesis)
            }
            ')' => {
                self.consume_curr();
                Some(Token::RightParenthesis)
            }
            '[' => {
                self.consume_curr();
                Some(Token::LeftBracket)
            }
            ']' => {
                self.consume_curr();
                Some(Token::RightBracket)
            }
            '\t' => {
                self.consume_curr();
                Some(Token::Tab)
            }
            ' ' => {
                self.consume_curr();
                Some(Token::Space)
            }
            '\n' => {
                self.consume_curr();
                Some(Token::Newline)
            }

            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn read_op_tokens() {
        let contents = r#"
            + 
            += 
            - 
            -= 
            * 
            *= 
            / 
            /= 
            % 
            %= 
            & 
            &= 
            && 
            &&= 
            | 
            |= 
            || 
            ||= 
            ~ 
            ~= 
            = 
            == 
            ! 
            != 
            < 
            <= 
            > 
            >=
        "#;

        let mut token_reader = Lexer::without_whitespaces(Cursor::new(contents));
        assert_eq!(token_reader.next(), Some(Token::Plus));
        assert_eq!(token_reader.next(), Some(Token::PlusAssign));
        assert_eq!(token_reader.next(), Some(Token::Minus));
        assert_eq!(token_reader.next(), Some(Token::MinusAssign));
        assert_eq!(token_reader.next(), Some(Token::Asterisk));
        assert_eq!(token_reader.next(), Some(Token::AsteriskAssign));
        assert_eq!(token_reader.next(), Some(Token::Divide));
        assert_eq!(token_reader.next(), Some(Token::DivideAssign));
        assert_eq!(token_reader.next(), Some(Token::Percetange));
        assert_eq!(token_reader.next(), Some(Token::PercetangeAssign));
        assert_eq!(token_reader.next(), Some(Token::Ampersand));
        assert_eq!(token_reader.next(), Some(Token::AmpersandAssign));
        assert_eq!(token_reader.next(), Some(Token::DoubleAmpersand));
        assert_eq!(token_reader.next(), Some(Token::DoubleAmpersandAssign));
        assert_eq!(token_reader.next(), Some(Token::VerticalBar));
        assert_eq!(token_reader.next(), Some(Token::VerticalBarAssign));
        assert_eq!(token_reader.next(), Some(Token::DoubleVerticalBar));
        assert_eq!(token_reader.next(), Some(Token::DoubleVerticalBarAssign));
        assert_eq!(token_reader.next(), Some(Token::Tilde));
        assert_eq!(token_reader.next(), Some(Token::TildeAssign));
        assert_eq!(token_reader.next(), Some(Token::Assign));
        assert_eq!(token_reader.next(), Some(Token::Equal));
        assert_eq!(token_reader.next(), Some(Token::ExclamationMark));
        assert_eq!(token_reader.next(), Some(Token::NotEqual));
        assert_eq!(token_reader.next(), Some(Token::LessThan));
        assert_eq!(token_reader.next(), Some(Token::LessEqual));
        assert_eq!(token_reader.next(), Some(Token::GreaterThan));
        assert_eq!(token_reader.next(), Some(Token::GreaterEqual));
        assert_eq!(token_reader.next(), None);
    }

    #[test]
    fn read_numbers() {
        let contents = "-323.2 22 -10 1222.";
        let mut token_reader = Lexer::without_whitespaces(Cursor::new(contents));

        assert_eq!(token_reader.next(), Some(Token::Minus));
        assert_eq!(token_reader.next(), Some(Token::Float(323.2)));
        assert_eq!(token_reader.next(), Some(Token::Integer(22)));
        assert_eq!(token_reader.next(), Some(Token::Minus));
        assert_eq!(token_reader.next(), Some(Token::Integer(10)));
        assert_eq!(token_reader.next(), Some(Token::Float(1222.)));
        assert_eq!(token_reader.next(), None);
    }

    #[test]
    fn read_words() {
        let contents = "contents let main _ var1 var2 var_33";
        let mut token_reader = Lexer::without_whitespaces(Cursor::new(contents));

        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("contents".to_owned()))
        );
        assert_eq!(
            token_reader.next(),
            Some(Token::ReservedWord("let".to_owned()))
        );
        assert_eq!(
            token_reader.next(),
            Some(Token::ReservedWord("main".to_owned()))
        );
        assert_eq!(
            token_reader.next(),
            Some(Token::ReservedWord("_".to_owned()))
        );
        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("var1".to_owned()))
        );
        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("var2".to_owned()))
        );
        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("var_33".to_owned()))
        );
        assert_eq!(token_reader.next(), None);
    }

    #[test]
    fn read_combination() {
        let contents = "let mut token_reader = TokenReader::new(Cursor::new(contents));";
        let mut token_reader = Lexer::without_whitespaces(Cursor::new(contents));

        assert_eq!(
            token_reader.next(),
            Some(Token::ReservedWord("let".to_owned()))
        );

        assert_eq!(
            token_reader.next(),
            Some(Token::ReservedWord("mut".to_owned()))
        );
        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("token_reader".to_owned()))
        );

        assert_eq!(token_reader.next(), Some(Token::Assign));
        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("TokenReader".to_owned()))
        );

        assert_eq!(token_reader.next(), Some(Token::ColonPath));
        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("new".to_owned()))
        );

        assert_eq!(token_reader.next(), Some(Token::LeftParenthesis));
        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("Cursor".to_owned()))
        );

        assert_eq!(token_reader.next(), Some(Token::ColonPath));
        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("new".to_owned()))
        );

        assert_eq!(token_reader.next(), Some(Token::LeftParenthesis));
        assert_eq!(
            token_reader.next(),
            Some(Token::Identifier("contents".to_owned()))
        );
        assert_eq!(token_reader.next(), Some(Token::RightParenthesis));
        assert_eq!(token_reader.next(), Some(Token::RightParenthesis));
        assert_eq!(token_reader.next(), Some(Token::SemiColon));
        assert_eq!(token_reader.next(), None);
    }

    #[test]
    fn read_combination2() {
        let expr = "true && (true || false)";
        let mut lexer = Lexer::without_whitespaces(Cursor::new(expr));

        assert_eq!(lexer.next(), Some(Token::ReservedWord("true".to_owned())));
        assert_eq!(lexer.next(), Some(Token::DoubleAmpersand));
        assert_eq!(lexer.next(), Some(Token::LeftParenthesis));
        assert_eq!(lexer.next(), Some(Token::ReservedWord("true".to_owned())));
        assert_eq!(lexer.next(), Some(Token::DoubleVerticalBar));
        assert_eq!(lexer.next(), Some(Token::ReservedWord("false".to_owned())));
        assert_eq!(lexer.next(), Some(Token::RightParenthesis));
        assert_eq!(lexer.next(), None);
    }
}
