use crate::error::SyntaxError;
use ez_lang::lexer::{FilteredLexer, Lexer};
use ez_lang::token::Token;
use std::fmt::Debug;
use std::io::Read;
use std::iter::Peekable;

pub type ScanResult<T> = Result<T, SyntaxError>;

#[derive(Debug)]
pub struct Parser<R>
where
    R: Read + Debug + Clone,
{
    lexer: Peekable<FilteredLexer<R>>,
}

/**
 * Parser para la gramatica:
 *  Expr   = Term { Opsuma Term }*
 *  Opsuma = + | -
 *  Term   = Factor { Opmult Factor }*
 *  Opmult = * | /
 *  Factor = (Expr) | FloatNumber | max(Expr, Expr)
 */
impl<R> Parser<R>
where
    R: Read + Debug + Clone,
{
    pub fn new(inner: R) -> Self {
        let lexer = Lexer::without_whitespaces(inner).peekable();

        Self { lexer }
    }

    fn program(&mut self) -> ScanResult {}

    fn expr(&mut self) -> ScanResult<f64> {
        let mut op1 = self.term()?;

        while let Some(tok) = self.lexer.peek() {
            match tok {
                Token::Plus => {
                    self.lexer.next();
                    op1 += self.term()?;
                }
                Token::Minus => {
                    self.lexer.next();
                    op1 -= self.term()?;
                }
                _ => break,
            }
        }

        Ok(op1)
    }

    fn term(&mut self) -> ScanResult<f64> {
        let mut op1 = self.factor()?;

        while let Some(tok) = self.lexer.peek() {
            match tok {
                Token::Asterisk => {
                    self.lexer.next();
                    op1 *= self.factor()?;
                }
                Token::Divide => {
                    self.lexer.next();
                    op1 /= self.factor()?;
                }
                _ => break,
            }
        }

        Ok(op1)
    }

    fn factor(&mut self) -> ScanResult<f64> {
        if let Some(tok) = self.lexer.next() {
            match tok {
                Token::Float(num) => Ok(num.parse().unwrap()),
                Token::Integer(num) => Ok(num.parse().unwrap()),
                Token::LeftParenthesis => {
                    let op = self.expr()?;
                    let tok = self.lexer.next();
                    match tok {
                        Some(Token::RightParenthesis) => Ok(op),
                        Some(tok) => Err(SyntaxError::new(tok.clone(), &[Token::RightParenthesis])),
                        None => Err(SyntaxError::new(
                            Token::EndOfFile,
                            &[Token::RightParenthesis],
                        )),
                    }
                }

                Token::Identifier(id) => {
                    if id != "max" {
                        Err(SyntaxError::new(
                            Token::Identifier(id.to_owned()),
                            &[Token::Identifier("max".to_owned())],
                        ))
                    } else {
                        let tok = self.lexer.next();
                        match tok {
                            Some(Token::LeftParenthesis) => {}
                            Some(tok) => {
                                return Err(SyntaxError::new(
                                    tok.clone(),
                                    &[Token::LeftParenthesis],
                                ));
                            }
                            None => {
                                return Err(SyntaxError::new(
                                    Token::EndOfFile,
                                    &[Token::LeftParenthesis],
                                ))
                            }
                        }

                        let op1 = self.expr()?;
                        let tok = self.lexer.next();
                        match tok {
                            Some(Token::Coma) => {}
                            Some(tok) => {
                                return Err(SyntaxError::new(tok.clone(), &[Token::Coma]));
                            }
                            None => return Err(SyntaxError::new(Token::EndOfFile, &[Token::Coma])),
                        }

                        let op2 = self.expr()?;
                        let tok = self.lexer.next();
                        match tok {
                            Some(Token::RightParenthesis) => {}
                            Some(tok) => {
                                return Err(SyntaxError::new(
                                    tok.clone(),
                                    &[Token::RightParenthesis],
                                ));
                            }
                            None => {
                                return Err(SyntaxError::new(
                                    Token::EndOfFile,
                                    &[Token::RightParenthesis],
                                ))
                            }
                        }

                        Ok(op1.max(op2))
                    }
                }

                tok => Err(SyntaxError::new(
                    tok.clone(),
                    &[
                        Token::Plus,
                        Token::Minus,
                        Token::Asterisk,
                        Token::Divide,
                        Token::LeftParenthesis,
                        Token::RightParenthesis,
                        Token::Float("".to_owned()),
                        Token::Integer("".to_owned()),
                    ],
                )),
            }
        } else {
            Err(SyntaxError::new(
                Token::EndOfFile,
                &[
                    Token::Plus,
                    Token::Minus,
                    Token::Asterisk,
                    Token::Divide,
                    Token::LeftParenthesis,
                    Token::RightParenthesis,
                    Token::Float("".to_owned()),
                    Token::Integer("".to_owned()),
                ],
            ))
        }
    }

    pub fn eval(&mut self) -> ScanResult<f64> {
        let result = self.expr()?;

        match self.lexer.peek() {
            Some(tok) => Err(SyntaxError::new(
                tok.clone(),
                &[Token::Plus, Token::Minus, Token::Asterisk, Token::Divide],
            )),

            _ => Ok(result),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io::Cursor;

    #[test]
    #[allow(clippy::float_cmp)]
    fn scan_test1() -> Result<(), Box<dyn Error>> {
        let expr = "(3+2)*5";
        let mut arithm_parser = ArithmParser::new(Cursor::new(expr));
        assert_eq!(arithm_parser.eval()?, 25.0);
        Ok(())
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn scan_test2() -> Result<(), Box<dyn Error>> {
        let expr = "((((3-2)+10)*5) - 10) * 2";
        let mut arithm_parser = ArithmParser::new(Cursor::new(expr));
        assert_eq!(arithm_parser.eval()?, 90.0);
        Ok(())
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn scan_test3() -> Result<(), Box<dyn Error>> {
        let expr = "(3+2)*(5-1)";
        let mut arithm_parser = ArithmParser::new(Cursor::new(expr));
        assert_eq!(arithm_parser.eval()?, 20.0);
        Ok(())
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn scan_test4_should_fail() -> Result<(), Box<dyn Error>> {
        let expr = "(3+2)51";
        let mut arithm_parser = ArithmParser::new(Cursor::new(expr));
        assert!(arithm_parser.eval().is_err());
        Ok(())
    }
}
