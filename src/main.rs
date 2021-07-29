extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::io::{self, Write};
use std::{env, fs};

pub mod ast;

pub mod eval;
use eval::{eval, eval_with_env, Environment};

pub mod native;

pub mod parser;
use parser::parse;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct RispPestParser;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() >= 2 {
		load_and_interpret(&args[1]);
	} else {
		repl();
	}
}

fn load_and_interpret(file_name: &String) {
	let script = fs::read_to_string(file_name);

	match script {
		Ok(s) => {
			let parse_tree = RispPestParser::parse(Rule::program, &s).unwrap();
			eval(parse(parse_tree));
		}
		Err(e) => {
			panic!("{:?}", e)
		}
	}
}

fn repl() {
	let mut env = Environment::new();

	loop {
		print!(">>> ");
		io::stdout().flush().unwrap();

		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		line = line.trim().to_string();

		if line.is_empty() {
			continue;
		}

		let parse_tree = RispPestParser::parse(Rule::program, &line);

		match parse_tree {
			Ok(tree) => {
				eval_with_env(parse(tree), &mut env);
			}
			Err(e) => {
				println!("{}", e)
			}
		}

		io::stdout().flush().unwrap();
	}
}
