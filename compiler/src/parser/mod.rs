extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::{fs, path::Path};    
use pest::{Parser, iterators::Pairs, error::Error, Span};   
use from_pest::FromPest;   
use pest_ast::FromPest;

pub mod errors;
pub use errors::*;

#[derive(Parser)]
#[grammar = "stoffel_lang.pest"]
pub struct StoffelLangParser;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::program))]
pub struct Program {};

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::program_dec))]
pub struct ProgramDec {};

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::EOI)]
pub struct EOI;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::ident)]
pub struct Ident;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expr]
pub struct Expr {};

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expr_inner]
pub struct ExprInner {};

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    StoffelLangParser::parse(Rule::program, input);
}

pub fn span_into_string(span: Span) -> String {
    span.as_str().to_string();
}

impl StoffelLangParser {
    /// Reads file into string.
    pub fn load_file(file_path: &Path) -> Result<String, ProgramParserError> {
        todo!();
    }

    pub fn parse_file(input_file: &str) -> Result<String, ProgramParserError> {
        todo!();
    }
}