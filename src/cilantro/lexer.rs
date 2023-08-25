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

type Span<'a> = nom_locate::LocatedSpan<&'a str>;



fn ws<'a, F, O, E>(parser: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E> 
where 
    E: ParseError<Span<'a>>,
    F: nom::Parser<Span<'a>, O, E>
{
    delimited(multispace0, parser, multispace0)
}


fn int (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        many1(
            terminated(digit1, many0(char('_')))
        ),
        |v: Vec<Span>| -> Result<Token, std::num::ParseIntError> {
            let s = v.iter().fold(String::new(), |mut a, i| {a.push_str(i); a});
            let n = s.parse::<i32>()?;

            Ok(Token {
                start: v[0].location_offset(),
                end: 0,
                t: TokenT::INT(n)
            })
        }
    ))
        (input)
}


fn bol (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        alt((tag("true"), tag("false"))),
        |s: Span| -> Result<Token, std::str::ParseBoolError> {
            let b = s.parse::<bool>()?;

            Ok(Token {
                start: s.location_offset(),
                end: 0,
                t: TokenT::BOOL(b)
            })
        }
    ))(input)
}


fn equal (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        many1(tag("=")),
        |v: Vec<Span>| -> Result<Token, nom::error::Error<Span>> {
            let t = match v.len() {
                1 => TokenT::EQ_1, 
                2 => TokenT::EQ_2, 
                _   => unreachable!()
            };
            Ok(Token {
                start: v[0].location_offset(),
                end: 0,
                t
            })
        }
    ))(input)
}


fn paren(input: Span) -> IResult<Span, Token> {
    ws(map_res(
        alt((tag("("), tag(")"))),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let t = match s.fragment() {
                &"(" => TokenT::PAREN_L,
                &")" => TokenT::PAREN_R,
                _   => unreachable!()
            };
            Ok(Token {
                start: s.location_offset(),
                end: 0,
                t
            })
        }
    ))(input)
}


fn keyword<'a>(keyword: &'static str, token_t: TokenT) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, Token, nom::error::Error<Span<'a>>> 
{
    ws(map_res(
        tag(keyword),
        move |s: Span| -> Result<Token, nom::error::Error<Span<'a>>> {
            Ok(Token {
                start: s.location_offset(),
                end: 0,
                t: token_t.clone()
            })
        }
    ))
}


fn ident (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        recognize(
            pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_"))))
            )
        ),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            Ok(Token {
                start: s.location_offset(),
                end: 0,
                t: TokenT::IDENT(s.into_fragment().to_owned())
            })
        }
    ))(input)
}


pub fn tokenize (source: String) -> Vec<Token> {

    // NOTE: Ident has to be placed *AFTER* keywords, otherwise it will treat every keyword as an
    // identifier.
    
    let span = Span::new(&source);
    let parsers = (
        int,
        bol,
        equal,
        paren,
        keyword("let", TokenT::K_LET),
        ident,
    );
    let mut parser = many1(alt(parsers));
    let res = parser(span).unwrap();
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
