use nom::{
    IResult,
    branch::alt,
    error::ParseError,
    combinator::{map_res},
    multi::{many1, many0},
    bytes::complete::{tag},
    character::complete::{char, digit1, multispace0}, 
    sequence::{terminated, delimited}
};

use super::*;

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
        |s: &str| -> Result<Token, std::str::ParseBoolError> {
            let b = s.parse::<bool>()?;

            Ok(Token {
                start: 0,
                end: 0,
                t: TokenT::BOOL(b)
            })
        }
    ))(input)
}

pub fn tokenize (source: String) -> Vec<Token> {
    let mut parser = many0(alt((int, bol)));
    let tokens = parser(&source).unwrap().1;
    tokens
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test () {
        let v = tokenize("  100   false 68_104  truetrue".to_owned());
        assert!(matches!(v[0].t, TokenT::INT(100)));
        assert!(matches!(v[1].t, TokenT::BOOL(false)));
        assert!(matches!(v[2].t, TokenT::INT(68104)));
        assert!(matches!(v[3].t, TokenT::BOOL(true)));
        assert!(matches!(v[4].t, TokenT::BOOL(true)));
    }
}
