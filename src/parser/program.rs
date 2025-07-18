use crate::parser::ast::ast::Program;
use crate::parser::ast::common::Span;
use crate::parser::element::element_parser;
use nom::IResult;
use nom::character::complete::multispace0;
use nom::combinator::{all_consuming, map};
use nom::multi::many0;
use nom::sequence::terminated;

pub fn program_parser(input: Span) -> IResult<Span, Program> {
    map(
        all_consuming(terminated(many0(element_parser), multispace0)),
        |body| Program { body },
    )(input)
}
