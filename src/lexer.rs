use nom::{
    IResult,
    branch::alt,
    error::ParseError,
    combinator::{map_res, recognize},
    multi::{many1, many0},
    bytes::complete::{tag, take_until},
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
            let n = s.parse::<i64>()?;

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

fn str_lit (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        delimited(char('"'), take_until("\""), char('"')),
        |s: Span| -> Result<Token, std::str::ParseBoolError> {
            Ok(Token {
                start: s.location_offset(),
                end: s.location_offset() + s.len(),
                data: TokenData::STR_LIT(s.to_string())
            })
        }
    ))(input)
}


// Used by library signature annotation parser.
pub fn types (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        alt((
            tag("i64"),
            tag("void"), 
            tag("str"),
        )),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let t = match s.fragment() {
                &"i64"  => Type::Int,
                &"void" => Type::Void,
                &"str" => Type::String,
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


fn tags (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        alt((
            tag("="),
            tag("->")
        )),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let data = match s.fragment() {
                &"="  => TokenData::ASSIGN,
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


fn symbols (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        recognize(one_of("(){},:")),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let data = match s.fragment() {
                &"(" => TokenData::PAREN_L,
                &")" => TokenData::PAREN_R,
                &"{" => TokenData::CURLY_L,
                &"}" => TokenData::CURLY_R,
                &"," => TokenData::COMMA,
                &":" => TokenData::COLON,
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


fn op_tags (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        recognize(alt((
            tag("=="),
            tag("!="),
            tag("&&"),
            tag("||")
        ))),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let data = match s.fragment() {
                &"==" => TokenData::OP1_b("==".to_owned()),
                &"!=" => TokenData::OP1_b("!=".to_owned()),
                &"&&" => TokenData::OP2_b("&&".to_owned()),
                &"||" => TokenData::OP2_b("||".to_owned()),
                s @ _ => panic!("Unknown operator tag '{}'", s)
            };
            Ok(Token {
                start: s.location_offset(),
                end: s.location_offset() + s.len(),
                data
            })
        }
    ))(input)
}


fn op_symbols (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        recognize(one_of("/*+-")),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            let data = match s.fragment() {
                &"*" => TokenData::OP4_n("*".to_owned()),
                &"/" => TokenData::OP4_n("/".to_owned()),
                &"+" => TokenData::OP3_n("+".to_owned()),
                &"-" => TokenData::OP3_n("-".to_owned()),
                s @ _ => panic!("Unknown operator symbol '{}'", s)
            };
            Ok(Token {
                start: s.location_offset(),
                end: s.location_offset() + s.len(),
                data 
            })
        }
    ))(input)
}

fn op_unary (input: Span) -> IResult<Span, Token> {
    ws(map_res(
        recognize(one_of("!")),
        |s: Span| -> Result<Token, nom::error::Error<Span>> {
            Ok(Token {
                start: s.location_offset(),
                end: s.location_offset() + s.len(),
                data: TokenData::OP_UNARY(s.to_string())
            })
        }
    ))(input)
}




pub fn tokenize (source: String) -> Tokens {
    let span = Span::new(&source);

    // NOTE: Rule for parser order. More general parsers should go in the bottom, that way the more
    // specific ones will filter first, before the general ones capture it.
    
    let parsers = (
        keyword("let", TokenData::K_LET),
        keyword("func", TokenData::K_FUNC),
        keyword("return", TokenData::K_RETURN),
        keyword("if", TokenData::K_IF),

        bol,
        types,
        int,

        op_tags,
        tags,
        op_unary,
        op_symbols,
        symbols,

        str_lit,
        ident,
    );
    let mut parser = many1(alt(parsers));
    let res = parser(span).unwrap();

    // Some unrecognized token
    if !res.0.is_empty() {
        let source = res.0;

        let pos = if let Some(last) = res.1.last() { 
            last.end+1
        } else { 0 };

        let mut end = 0;
        for _ in 0..20 {
            if end == source.len() || source.as_bytes()[end].is_ascii_control() { 
                break
            }
            end += 1;
        }

        println!("=== Tokenization Error ===");
        println!("unrecognized token at: {pos}");
        println!("  |  {}", &source[..end]);
        println!("     ^---- here");
        panic!("- lexer did not consume the entire string. Unrecognized token.");
    }
    let mut tokens = res.1;
    tokens.push(Token{ start: source.len(), end: source.len(), data: TokenData::EOF});
    tokens
}

