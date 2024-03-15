use cssparser::{
    BasicParseError, BasicParseErrorKind, ParseError, Parser, Token,
};

#[derive(Debug, PartialEq)]
struct Variant<'a> {
    raw: String,
    kind: VariantKind<'a>,
}

#[derive(Debug, PartialEq)]
enum VariantKind<'a> {
    Arbitrary(ArbitraryVariant),
    Literal(LiteralVariant<'a>),
}

// TODO: name better
// Replacement: &:nth-child(3)
// Nested: @media
#[derive(Debug, PartialEq)]
enum ArbitraryVariantKind {
    Replacement,
    Nested,
}

// Something like [@media(min-width: 300px)] or [&:nth-child(3)]
#[derive(Debug, PartialEq)]
struct ArbitraryVariant {
    kind: ArbitraryVariantKind,
    value: String,
}

#[derive(Debug, PartialEq)]
enum Modifier<'a> {
    Arbitrary(Token<'a>),
    Literal(String),
}

#[derive(Debug, PartialEq)]
struct LiteralVariant<'a> {
    value: String,
    modifier: Option<Modifier<'a>>,
    arbitrary: Option<String>,
}

enum ParserError {
    UnexpectedToken,
    UnexpectedEnd,
}

impl<'i> ArbitraryVariant {
    fn parse<'a>(
        parser: &mut Parser<'i, 'a>,
    ) -> Result<Self, ParseError<'a, ()>> {
        let start = parser.state();
        parser.parse_nested_block(|parser| loop {
            match parser.next() {
                Err(BasicParseError {
                    kind: BasicParseErrorKind::EndOfInput,
                    ..
                }) => {
                    return Ok(Self {
                        kind: ArbitraryVariantKind::Nested,
                        value: parser
                            .slice(start.position()..parser.position())
                            .to_string(),
                    });
                }
                Ok(Token::AtKeyword(at_rule)) => {
                    println!("at_rule: {at_rule:?}");
                }
                other => {
                    println!("other: {:?}", other);
                }
            }
        })
    }
}

fn parse_arbitrary<'i, 'a>(
    parser: &mut Parser<'i, 'a>,
) -> Result<String, ParseError<'a, ()>> {
    let start = parser.state();
    while let Err(e) = parser.next() {
        match e.kind {
            BasicParseErrorKind::EndOfInput => {
                let value = parser.slice(start.position()..parser.position());
                println!("{:?}", value);
                return Ok(value.get(..value.len() - 1).unwrap().into());
            }
            _ => {
                parser.reset(&start);
                return Err(parser.new_custom_error(()));
            }
        }
    }
    Err(parser.new_custom_error(()))
}

impl<'i> Variant<'i> {
    fn parse<'a>(
        parser: &mut Parser<'i, 'a>,
    ) -> Result<Self, ParseError<'a, ()>> {
        let mut is_first_token = true;
        let mut ident = String::new();
        let mut arbitrary: Option<String> = None;

        let start_state = parser.state();

        loop {
            match parser.next() {
                Ok(Token::Colon) => {
                    break;
                }
                Ok(Token::SquareBracketBlock) => {
                    if is_first_token {
                        // trait as ArbitraryVariant
                        let arbitrary_variant =
                            parser.try_parse(ArbitraryVariant::parse)?;
                        parser.expect_colon();
                        return Ok(Self {
                            raw: parser
                                .slice(
                                    start_state.position()..parser.position(),
                                )
                                .into(),
                            kind: VariantKind::Arbitrary(arbitrary_variant),
                        });
                    } else {
                        arbitrary = parser
                            .parse_nested_block(|parser| {
                                let start = parser.state();
                                while let e = parser.next() {
                                    match e {
                                        Err(BasicParseError {
                                            kind:
                                                BasicParseErrorKind::EndOfInput,
                                            ..
                                        }) => {
                                            return Ok(parser
                                                .slice(
                                                    start.position()
                                                        ..parser.position(),
                                                )
                                                .into());
                                        }
                                        Ok(_) => {}
                                        _ => {
                                            return Err(parser
                                                .new_custom_error::<(), ()>(
                                                    (),
                                                ));
                                        }
                                    }
                                }
                                Err(parser.new_custom_error(()))
                            })
                            .ok();
                    }
                }
                Ok(Token::Delim('/')) => {
                    // modifier = parser.try_parse(Modifier::parse);
                }
                Ok(Token::Ident(id)) => {
                    ident = id.to_string();
                }
                Ok(token @ Token::Delim('@')) => {
                    ident += "@";
                }
                Ok(Token::AtKeyword(at_rule)) => {
                    println!("{:?}", at_rule);
                    ident += "@";
                    ident += at_rule;
                }
                Ok(token) => {
                    println!("{:?}", token);
                }
                Err(e) => {
                    parser.reset(&start_state);
                    return Err(parser.new_custom_error(()));
                }
            }
            is_first_token = false;
        }

        // remove the last `-` if it exists
        if ident.ends_with('-') {
            ident.pop();
        }

        Ok(Self {
            raw: parser
                .slice(start_state.position()..parser.position())
                .into(),
            kind: VariantKind::Literal(LiteralVariant {
                value: ident,
                modifier: None,
                arbitrary,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::{Parser, ParserInput};

    fn create_variant(input: &str) -> Variant {
        let mut input = ParserInput::new(input);
        let mut parser = Parser::new(&mut input);
        Variant::parse(&mut parser).unwrap()
    }

    #[test]
    fn test_plain_variant() {
        assert_eq!(
            create_variant("group-hover:"),
            Variant {
                raw: "group-hover:".into(),
                kind: VariantKind::Literal(LiteralVariant {
                    value: "group-hover".into(),
                    modifier: None,
                    arbitrary: None,
                })
            }
        );
    }

    #[test]
    fn test_arbitrary_variant() {
        assert_eq!(
            create_variant("[@media(min-width:200px)]:"),
            Variant {
                raw: "[@media(min-width:200px)]:".into(),
                kind: VariantKind::Arbitrary(ArbitraryVariant {
                    kind: ArbitraryVariantKind::Nested,
                    value: "@media(min-width:200px)".into(),
                })
            }
        );
    }

    // group-[&:hover]
    #[test]
    fn test_literal_variant_with_arbitrary() {
        assert_eq!(
            create_variant("group-[&:hover]:"),
            Variant {
                raw: "group-[&:hover]:".into(),
                kind: VariantKind::Literal(LiteralVariant {
                    value: "group".into(),
                    modifier: None,
                    arbitrary: Some("&:hover".into()),
                })
            }
        );
    }

    // @md
    #[test]
    fn test_at() {
        assert_eq!(
            create_variant("@md:"),
            Variant {
                raw: "@md:".into(),
                kind: VariantKind::Literal(LiteralVariant {
                    value: "@md".into(),
                    modifier: None,
                    arbitrary: None,
                })
            }
        );
    }
}
