use std::borrow::Cow;

use nom::{
    bytes::complete::{tag, take, take_until, take_while1},
    combinator::{opt, verify},
    sequence::delimited,
    IResult,
};

use crate::elements::Element;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Macros<'a> {
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "ser", serde(skip_serializing_if = "Option::is_none"))]
    pub arguments: Option<Cow<'a, str>>,
}

impl Macros<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, _) = tag("{{{")(input)?;
        let (input, name) = verify(
            take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            |s: &str| s.starts_with(|c: char| c.is_ascii_alphabetic()),
        )(input)?;
        let (input, arguments) = opt(delimited(tag("("), take_until(")}}}"), take(1usize)))(input)?;
        let (input, _) = tag("}}}")(input)?;

        Ok((
            input,
            Element::Macros(Macros {
                name: name.into(),
                arguments: arguments.map(Into::into),
            }),
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Macros::parse("{{{poem(red,blue)}}}"),
        Ok((
            "",
            Element::Macros(Macros {
                name: "poem".into(),
                arguments: Some("red,blue".into())
            })
        ))
    );
    assert_eq!(
        Macros::parse("{{{poem())}}}"),
        Ok((
            "",
            Element::Macros(Macros {
                name: "poem".into(),
                arguments: Some(")".into())
            })
        ))
    );
    assert_eq!(
        Macros::parse("{{{author}}}"),
        Ok((
            "",
            Element::Macros(Macros {
                name: "author".into(),
                arguments: None
            })
        ))
    );
    assert!(Macros::parse("{{{0uthor}}}").is_err());
    assert!(Macros::parse("{{{author}}").is_err());
    assert!(Macros::parse("{{{poem(}}}").is_err());
    assert!(Macros::parse("{{{poem)}}}").is_err());
}
