use std::str::ParseBoolError;

use nom::{
    IResult,
    branch::alt,
    error::ParseError,
    combinator::{map_res, recognize},
    multi::{many1, many0},
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, alpha1, alphanumeric1}, 
    sequence::{terminated, delimited, pair}
};

use super::*;

#[derive(Debug)]
struct Error {
    s: Vec<(String, nom::error::ErrorKind)>
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer error")
    }
}
impl std::error::Error for Error {}
impl nom::error::ParseError<&str> for Error {
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        Self {
            s: vec![(input[..10].to_owned(), kind)]
        }
    }
    fn append(input: &str, kind: nom::error::ErrorKind, other: Self) -> Self {
        let mut s = other.s;
        s.push((input[..10].to_owned(), kind));
        Self { s }       
    }
}



fn ws<'a, F, O, E: ParseError<&'a str>>(parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> 
where 
    F: nom::Parser<&'a str, O, E>
{
    delimited(multispace0, parser, multispace0)
}


fn int (input: &str) -> IResult<&str, Token> {
    ws(map_res(
        many1(
            terminated(digit1, many0(char('_')))
        ),
        |v: Vec<&str>| -> Result<Token, std::num::ParseIntError> {
            let s = v.iter().fold(String::new(), |mut a, i| {a.push_str(i); a});
            let n = s.parse::<i32>()?;

            Ok(Token {
                start: 0,
                end: 0,
                t: TokenT::INT(n)
            })
        }
    ))
        (input)
}


fn bol (input: &str) -> IResult<&str, Token> {
    ws(map_res(
        alt((tag("true"), tag("false"))),
        |s: &str| -> Result<Token, ParseBoolError> {
            let b = s.parse::<bool>()?;

            Ok(Token {
                start: 0,
                end: 0,
                t: TokenT::BOOL(b)
            })
        }
    ))(input)
}


fn equal (input: &str) -> IResult<&str, Token> {
    ws(map_res(
        many1(char('=')),
        |v: Vec<_>| -> Result<Token, Error> {
            let t = match v.len() {
                1 => TokenT::EQ_1, 
                2 => TokenT::EQ_2, 
                _   => unreachable!()
            };
            Ok(Token {
                start: 0,
                end: 0,
                t
            })
        }
    ))(input)
}


fn paren(input: &str) -> IResult<&str, Token> {
    ws(map_res(
        alt((char('('), char(')'))),
        |c: char| -> Result<Token, nom::error::Error<&str>> {
            let t = match c {
                '(' => TokenT::PAREN_L,
                ')' => TokenT::PAREN_R,
                _   => unreachable!()
            };
            Ok(Token {
                start: 0,
                end: 0,
                t
            })
        }
    ))(input)
}


fn keyword<'a>(keyword: &'static str, token_t: TokenT) -> impl FnMut(&'a str) -> IResult<&'a str, Token, nom::error::Error<&'a str>> 
{
    ws(map_res(
        tag(keyword),
        move |_: &str| -> Result<Token, nom::error::Error<&str>> {
            Ok(Token {
                start: 0,
                end: 0,
                t: token_t.clone()
            })
        }
    ))
}


fn ident (input: &str) -> IResult<&str, Token> {
    ws(map_res(
        recognize(
            pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_"))))
            )
        ),
        |s: &str| -> Result<Token, Error> {
            Ok(Token {
                start: 0,
                end: 0,
                t: TokenT::IDENT(s.to_owned())
            })
        }
    ))(input)
}


pub fn tokenize (source: String) -> Vec<Token> {

    // NOTE: Ident has to be placed *AFTER* keywords, otherwise it will treat every keyword as an
    // identifier.
    
    let parsers = (
        int,
        bol,
        equal,
        paren,
        keyword("let", TokenT::K_LET),
        ident,
    );
    let mut parser = many1(alt(parsers));
    let res = parser(&source).unwrap();
    if !res.0.is_empty() {
        println!("{}", res.0);
        panic!("Lexer did not consume the entire string.");
    }
    let tokens = res.1;
    tokens
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test () {
        let v = tokenize("let a = 100\n let b = 68_104".to_owned());
        println!("{:?}", v);
        assert!(matches!(v[0].t, TokenT::K_LET));
        match &v[1].t {
            TokenT::IDENT(s) => s == "a",
            _ => panic!()
        };
        assert!(matches!(v[2].t, TokenT::EQ_1));
        assert!(matches!(v[3].t, TokenT::INT(100)));
        assert!(matches!(v[4].t, TokenT::K_LET));
        match &v[5].t {
            TokenT::IDENT(s) => s == "b",
            _ => panic!()
        };
        assert!(matches!(v[6].t, TokenT::EQ_1));
        assert!(matches!(v[7].t, TokenT::INT(68104)));
    }
}
