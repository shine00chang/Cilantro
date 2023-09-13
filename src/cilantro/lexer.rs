use nom::{
    IResult,
    branch::alt,
    error::ParseError,
    combinator::{map_res, recognize},
    multi::{many1, many0},
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, alpha1, alphanumeric1, one_of}, 
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
                end: v[0].location_offset() + v.last().unwrap().len(),
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
                end: s.location_offset() + s.len(),
                data: TokenData::BOOL(b)
            })
        }
    ))(input)
}


// Used by library signature annotation parser.
pub fn types (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        alt((
            tag("i32"),
            tag("void"), 
        )),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let t = match s.fragment() {
                &"i32"  => Type::Int,
                &"void" => Type::Void,
                _   => unreachable!()
            };
            Ok(Token {
                start: s.location_offset(),
                end: s.location_offset() + s.len(),
                data: TokenData::TYPE(t)
            })
        }
    ))(input)
}


fn symbols (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        alt((
            tag("="),
            tag("=="), 
            tag("->")
        )),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let data = match s.fragment() {
                &"="  => TokenData::EQ_1,
                &"==" => TokenData::EQ_2,
                &"->" => TokenData::ARROW,
                _   => unreachable!()
            };
            Ok(Token {
                start: s.location_offset(),
                end: s.location_offset() + s.len(),
                data 
            })
        }
    ))(input)
}


fn characters (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        recognize(one_of("(){},")),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let data = match s.fragment() {
                &"(" => TokenData::PAREN_L,
                &")" => TokenData::PAREN_R,
                &"{" => TokenData::CURLY_L,
                &"}" => TokenData::CURLY_R,
                &"," => TokenData::COMMA,
                _   => unreachable!()
            };
            Ok(Token {
                start: s.location_offset(),
                end: s.location_offset() + s.len(),
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
                end: s.location_offset() + s.len(),
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
                end: s.location_offset() + s.len(),
                data: TokenData::IDENT(s.into_fragment().to_owned())
            })
        }
    ))(input)
}


fn num_op_p1 (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        recognize(one_of("+-")),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            Ok(Token {
                start: s.location_offset(),
                end: s.location_offset() + s.len(),
                data: TokenData::NUMOP_1(s.fragment().to_string())
            })
        }
    ))(input)
}

fn num_op_p2 (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        alt((tag("*"), tag("/"))),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            Ok(Token {
                start: s.location_offset(),
                end: s.location_offset() + s.len(),
                data: TokenData::NUMOP_2(s.fragment().to_string())
            })
        }
    ))(input)
}



pub fn tokenize (source: String) -> Tokens {

    let span = Span::new(&source);
    let parsers = (
        symbols,
        characters,
        num_op_p1,
        num_op_p2,
        types,
        int,
        bol,

        // NOTE: Ident has to be placed *AFTER* keywords, otherwise it will treat every keyword as an
        // identifier.
        keyword("let", TokenData::K_LET),
        keyword("func", TokenData::K_FUNC),
        keyword("return", TokenData::K_RETURN),

        ident,
    );
    let mut parser = many1(alt(parsers));
    let res = parser(span).unwrap();
    if !res.0.is_empty() {
        println!("{}", res.0);
        panic!("Lexer did not consume the entire string.");
    }
    let mut tokens = res.1;
    tokens.push(Token{ start: source.len(), end: source.len(), data: TokenData::EOF});
    tokens
}

