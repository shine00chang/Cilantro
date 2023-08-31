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
                data: TokenData::INT(n)
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
                data: TokenData::BOOL(b)
            })
        }
    ))(input)
}


fn equal (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        many1(tag("=")),
        |v: Vec<Span>| -> Result<Token, nom::error::Error<Span>> {
            let data = match v.len() {
                1 => TokenData::EQ_1, 
                2 => TokenData::EQ_2, 
                _   => unreachable!()
            };
            Ok(Token {
                start: v[0].location_offset(),
                end: 0,
                data
            })
        }
    ))(input)
}


fn paren(input: Span) -> IResult<Span, Token> {
    ws(map_res(
        alt((tag("("), tag(")"))),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let data = match s.fragment() {
                &"(" => TokenData::PAREN_L,
                &")" => TokenData::PAREN_R,
                _   => unreachable!()
            };
            Ok(Token {
                start: s.location_offset(),
                end: 0,
                data 
            })
        }
    ))(input)
}


fn keyword<'a>(keyword: &'static str, token: TokenData) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, Token, nom::error::Error<Span<'a>>> 
{
    ws(map_res(
        tag(keyword),
        move |s: Span| -> Result<Token, nom::error::Error<Span<'a>>> {
            Ok(Token {
                start: s.location_offset(),
                end: 0,
                data: token.clone()
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
                data: TokenData::IDENT(s.into_fragment().to_owned())
            })
        }
    ))(input)
}

fn a (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        tag("a"),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            Ok(Token {
                start: s.location_offset(),
                end: 0,
                data: TokenData::a('a')
            })
        }
    ))(input)
}

fn b (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        tag("b"),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            Ok(Token {
                start: s.location_offset(),
                end: 0,
                data: TokenData::b('b')
            })
        }
    ))(input)
}




pub fn tokenize (source: String) -> Tokens {

    // NOTE: Ident has to be placed *AFTER* keywords, otherwise it will treat every keyword as an
    // identifier.
    
    let span = Span::new(&source);
    let parsers = (
        // Tests
        a,
        b,

        int,
        bol,
        equal,
        paren,
        keyword("let", TokenData::K_LET),
        ident,
    );
    let mut parser = many1(alt(parsers));
    let res = parser(span).unwrap();
    if !res.0.is_empty() {
        println!("{}", res.0);
        panic!("Lexer did not consume the entire string.");
    }
    let mut tokens = res.1;
    tokens.push(Token{ start: source.len(), end: 0, data: TokenData::EOF});
    tokens
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test () {
        let s = "let A = 100\n let B = 68_104".to_owned();
        let v = tokenize(s.clone());

        println!("{}", visualizer::print_tokens(&v, &s).unwrap());

        assert!(v[0].data == TokenData::K_LET);
        assert!(v[1].data == TokenData::IDENT("A".to_owned()));
        assert!(v[2].data == TokenData::EQ_1);
        assert!(v[3].data == TokenData::INT(100));
        assert!(v[4].data == TokenData::K_LET);
        assert!(v[5].data == TokenData::IDENT("B".to_owned()));
        assert!(v[6].data == TokenData::EQ_1);
        assert!(v[7].data == TokenData::INT(68104));
    }

    #[test]
    fn test2 () {
        let s = "baab".to_owned();
        let v = tokenize(s.clone());

        println!("{}", visualizer::print_tokens(&v, &s).unwrap());

        assert!(v[0].data == TokenData::b('b'));
        assert!(v[1].data == TokenData::a('a'));
        assert!(v[2].data == TokenData::a('a'));
        assert!(v[3].data == TokenData::b('b'));
    }
}
