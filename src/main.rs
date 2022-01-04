use std::{env, fs};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use environment::Environment;
use eval::{eval, eval_expr};
use parser::{parse, parse_expr_entry};
use crate::doc::Entry;
use crate::eval::eval_program;
use crate::val::Val;

pub mod ast;
pub mod doc;
pub mod environment;
pub mod eval;
pub mod stdlib;
pub mod parser;
pub mod scanner;
pub mod val;

#[cfg(test)]
mod tests;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() >= 2 { // first arg is always the name of the executable unless it's a special option
		match args[1].as_str() {
			"--dump-docs" => dump_docs(),
			file_name => load_and_interpret(file_name)
		}
	} else {
		repl()
	}
}

pub fn load_into(file_name: &str, env: &mut Environment) -> Result<Vec<Val>, String> {
	let script = fs::read_to_string(file_name);

	match script {
		Ok(s) => {
			let parse_tree = parse(s).unwrap();
			env.set_curr_module(file_name);
			match eval_program(parse_tree, env) {
				Ok(o) => Ok(o),
				Err(e) => Err(e)
			}
		}
		Err(e) => Err(e.to_string())
	}
}

fn dump_docs() {
	let env = Environment::new();

	let docs = Path::new("docs/modules");
	if docs.exists() {
		fs::remove_dir_all(docs).expect(format!("Failed to delete previous docs!").as_str());
	}
	fs::create_dir(docs);

	// There's definitely a better ways to do this, but it's enough for now
	for (_, entry) in env.get_doc().get_entries() {
		let module_path = entry.module.as_str();

		let module = &module_path[11..(module_path.len() - 5)].to_string().clone();
		let doc_path = format!("docs/modules/{}.md", module);

		if !Path::new(doc_path.as_str()).exists() {
			let mut file = File::create(doc_path.as_str()).unwrap();
			file.write_all(format!("This is the documentation for `{}`.\n\n", module).as_bytes()).unwrap();
			file.write_all(format!("---\n{}", entry).as_bytes()).unwrap();
			file.flush().unwrap();
		} else {
			let mut file = fs::OpenOptions::new()
				.write(true)
				.append(true)
				.open(doc_path.as_str())
				.unwrap();

			file.write_all(format!("---\n{}", entry).as_bytes()).unwrap();
			file.flush().unwrap();
		}
	}
}

fn load_and_interpret(file_name: &str) {
	let script = fs::read_to_string(file_name);

	match script {
		Ok(s) => {
			let parse_tree = parse(s).unwrap();
			match eval(parse_tree) {
				Ok(_) => {}
				Err(e) => eprintln!("{}", e)
			}
		}
		Err(e) => eprintln!("{:?}", e)
	}
}

fn repl() {
	let mut env = Environment::new();
	env.set_curr_module("<REPL>");

	loop {
		print!(">>> ");
		io::stdout().flush().unwrap();

		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		line = line.trim().to_string();

		if line.is_empty() {
			continue
		}

		let parse_tree = parse_expr_entry(line);

		match parse_tree {
			Ok(tree) =>
				match eval_expr(tree, &mut env) {
					Ok(val) => println!("{}", val),
					Err(e) => eprintln!("{}", e)
				}
			Err(e) => eprintln!("{:?}", e)
		}

		io::stdout().flush().unwrap();
	}
}
