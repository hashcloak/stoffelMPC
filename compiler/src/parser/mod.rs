extern crate pest;

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
pub struct Program {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::program_dec))]
pub struct ProgramDec {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::program_block))]
pub struct ProgramBlock {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::non_variable_declarations))]
pub struct NonVarDecs {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::EOI))]
pub struct EOI {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::ident))]
pub struct Ident {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expr))]
pub struct Expr {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expr_inner))]
pub struct ExprInner {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::if_exp))]
pub struct IfExpr {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::code_block))]
pub struct CodeBlock {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::literal_value))]
pub struct LiteralValue {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::parenthesized_expr))]
pub struct ParenthesizedExpr {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::struct_dec))]
pub struct StructDec {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::variable_dec))]
pub struct VariableDec {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::storage_dec))]
pub struct StorageDec {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::function_dec))]
pub struct FunctionDec {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::while_loop))]
pub struct WhileLoop {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::for_loop))]
pub struct ForLoop {}

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    StoffelLangParser::parse(Rule::program, input)
}

pub fn span_into_string(span: Span) -> String {
    span.as_str().to_string()
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