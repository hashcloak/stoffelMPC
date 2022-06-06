extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;    
use pest::{Parser, iterators::Pairs, error::Error}                                                                                                                                                                                     

#[derive(Parser)]
#[grammar = "stoffel_lang.pest"]
pub struct StoffelLangParser;

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    unimplemented!();
}