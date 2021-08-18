extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::io::{self, Write};
use std::{env, fs};

pub mod ast;

pub mod eval;
use eval::{eval, eval_expr, Environment};

pub mod native;

pub mod parser;
use parser::{parse, parse_expr};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PsilPestParser;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() >= 2 { // first arg is always the name of the executable
		load_and_interpret(&args[1])
	} else {
		repl()
	}
}

fn load_and_interpret(file_name: &String) {
	let script = fs::read_to_string(file_name);

	match script {
		Ok(s) => {
			let parse_tree = PsilPestParser::parse(Rule::program, &s).unwrap();
			match eval(parse(parse_tree)) {
				Ok(_) => {}
				Err(e) => eprintln!("{}", e)
			}
		}
		Err(e) => eprintln!("{:?}", e)
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
			continue
		}

		let parse_tree = PsilPestParser::parse(Rule::expr, &line);

		match parse_tree {
			Ok(tree) => {
				for pair in tree {
					match eval_expr(parse_expr(pair), &mut env) {
						Ok(val) => println!("{}", val),
						Err(e) => eprintln!("{}", e)
					}
				}
			}
			Err(e) => eprintln!("{}", e)
		}

		io::stdout().flush().unwrap();
	}
}
