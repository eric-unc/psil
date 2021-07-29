use crate::Rule;
use pest::iterators::{Pair, Pairs};

use crate::ast::*;

pub fn parse(tree: Pairs<Rule>) -> ProgramAst {
    parse_program(tree)
}

pub fn parse_program(tree: Pairs<Rule>) -> ProgramAst {
    println!("{}", tree);

    ProgramAst { expr_list: vec![] }
}
