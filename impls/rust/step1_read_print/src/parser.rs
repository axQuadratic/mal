use std::vec::Vec;

use crate::error::MalError;
use crate::error::MalResult as Result;
use crate::types::*;

// All types of tokens that can be expected to appear in a mal program
#[derive(Debug)]
enum Token {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    Quote,
    BackQuote,
    Caret,
    Tilde,
    AtSign,
    TildeAtSign,
    String(String),
    Comment(String),
    Atom(String)
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::OpenParen    => "(",
            Self::CloseParen   => ")",
            Self::OpenBracket  => "[",
            Self::CloseBracket => "]",
            Self::OpenCurly    => "{",
            Self::CloseCurly   => "}",
            Self::Quote        => "'",
            Self::BackQuote    => "`",
            Self::Caret        => "^",
            Self::Tilde        => "~",
            Self::AtSign       => "@",
            Self::TildeAtSign  => "~@",
            Self::Comment(s)   => s,
            Self::String(s)    => s,
            Self::Atom(s)      => s
        };

        write!(f, "{}", s)
    }
}


struct Tokenizer {
    data: Vec<char>,
    index: usize
}

impl Tokenizer {
    fn new(data: String) -> Self {
        Self {
            data: Vec::from_iter(data.chars()),
            index: 0
        }
    }
    
    fn peek(&mut self) -> Option<char> {
        self.data.get(self.index).copied()
    }

    fn consume(&mut self) -> Option<char> {
        let next = self.data.get(self.index).copied();
        self.index += 1;
        return next;
    }

    fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        'main: while let Some(next_token) = self.consume() {
            if next_token.is_whitespace() || next_token == ','{
                continue;
            }
            
            let token = match next_token {
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '[' => Token::OpenBracket,
                ']' => Token::CloseBracket,
                '{' => Token::OpenCurly,
                '}' => Token::CloseCurly,
                '\'' => Token::Quote,
                '`' => Token::BackQuote,
                '^' => Token::Caret,
                '@' => Token::AtSign,
                
                '~' => {
                    // Check for the ~@ construct before tokenising as a tilde
                    if self.peek() == Some('@') {
                        self.consume();
                        tokens.push(Token::TildeAtSign);
                    }
                    else {
                        tokens.push(Token::Tilde);
                    }

                    continue;
                },

                '"' => {
                    // Construct a string until the next unescaped double-quote
                    let mut s = String::new();
                    s.push(next_token);

                    while let Some(next_token) = self.consume() {
                        if next_token == '\\' {
                            // Backslashes escape any following double-quotes
                            if self.peek() == Some('"') {
                                s.push(self.consume().unwrap());
                            }
                            else {
                                s.push('\\');
                            }
                            
                            continue;
                        }

                        s.push(next_token);

                        if next_token != '"' {
                            continue;
                        }

                        tokens.push(Token::String(s));
                        continue 'main;
                    }

                    return Err("Unbalanced string".into());
                },

                ';' => {
                    // Anything after a comment character up to the next newline is captured
                    let mut s = String::new();
                    s.push(next_token);
                    
                    while let Some(next_token) = self.consume() {
                        if next_token == '\n' {
                            break;
                        }
                        
                        s.push(next_token);
                    }

                    tokens.push(Token::Comment(s));
                    continue;
                }

                _ => {
                    // Tokenise any other characters as an atom until a special character or whitespace is hit
                    let mut s = String::new();
                    s.push(next_token);

                    while let Some(next_token) = self.peek() {
                        if next_token.is_whitespace() {
                            break;
                        }

                        match next_token {
                            '[' | ']' | '{' | '}' | '(' | ')' | '\'' | '"' | '`' | ',' | ';' => break,

                            _ => ()
                        };

                        self.consume();
                        s.push(next_token);
                    }

                    tokens.push(Token::Atom(s));
                    continue;
                }
            };

            tokens.push(token);
        }

        Ok(tokens)
    }
}


struct Parser {
    tokens: Vec<Token>,
    index: usize
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0
        }
    }
    
    fn peek<'a>(&'a self) -> Option<&'a Token> {
        self.tokens.get(self.index)
    }

    fn consume<'a>(&'a mut self) -> Option<&'a Token> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        return token;
    }

    fn parse(&mut self) -> Result<Vec<Box<dyn MalValue>>> {
        let mut forms = vec![];

        // While tokens remain in the input data, keep looking for Lisp forms
        while let Some(next_token) = self.peek() {
            forms.push(self.parse_form()?);
        }

        return Ok(forms);
    }

    // Get the next Lisp form found in the parser; either a list or an atom
    fn parse_form(&mut self) -> Result<Box<dyn MalValue>> {
        let Some(next_token) = self.peek() else {
            return Err("Unexpected end of input".into());
        };

        match next_token {
            Token::OpenParen => {
                // This form is a list, start reading it from the next token
                self.consume();
                self.read_list()
            },

            // If the form does not match any other pattern, read it as an atom
            _ => self.read_list()
        }
    }

    // Read a list with the first token at the current position
    fn read_list(&mut self) -> Result<ListValue> {
        let list = ListValue::new();
        
        while let Some(next_token) = self.peek() {
            if *next_token == Token::CloseParen {
                // End of list
                self.consume();
                return Ok(list);
            }

            list.push(parse_form()?);
        }

        Err("Unbalanced list".into())
    }
}

pub fn parse_line(input: String) {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize().unwrap();
    
    let mut parser = Parser::new(tokens);
    let forms = parser.parse().unwrap();

    println!("{:?}", tokens);
}
