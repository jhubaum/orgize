use nom::{
    bytes::complete::{tag, take_while},
    combinator::verify,
    IResult,
};

use crate::elements::Element;

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Target<'a> {
    pub target: &'a str,
}

impl Target<'_> {
    #[inline]
    pub(crate) fn parse(input: &str) -> IResult<&str, Element<'_>> {
        let (input, _) = tag("<<")(input)?;
        let (input, target) = verify(
            take_while(|c: char| c != '<' && c != '\n' && c != '>'),
            |s: &str| s.starts_with(|c| c != ' ') && s.ends_with(|c| c != ' '),
        )(input)?;
        let (input, _) = tag(">>")(input)?;

        Ok((input, Element::Target(Target { target })))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Target::parse("<<target>>"),
        Ok(("", Element::Target(Target { target: "target" })))
    );
    assert_eq!(
        Target::parse("<<tar get>>"),
        Ok(("", Element::Target(Target { target: "tar get" })))
    );
    assert!(Target::parse("<<target >>").is_err());
    assert!(Target::parse("<< target>>").is_err());
    assert!(Target::parse("<<ta<get>>").is_err());
    assert!(Target::parse("<<ta>get>>").is_err());
    assert!(Target::parse("<<ta\nget>>").is_err());
    assert!(Target::parse("<<target>").is_err());
}