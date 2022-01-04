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

	// There's definitely a better way to do this, but it probably involves fancy lifetimes
	let mut docs: HashMap<&str, &mut Vec<&Entry>> = HashMap::new();

	for (_, entry) in env.get_doc().get_entries() {
		let module = entry.module.as_str();

		if docs.contains_key(module) {
			let entry_list = docs.get_mut(module).unwrap();
			entry_list.push(entry);
		} else {
			/*let mut new_entries = vec![entry];
			docs.insert(module, new_entries.borrow_mut());*/
		}
		//docs.insert(entry.module.clone(), entry);
	}

	for (module, entries) in docs {
		let mut doc = format!("This is the documentation for `{}`.\n", module);

		for entry in entries {
			doc.push_str(format!("\n---\n{}\n", entry).as_str());
		}

		// TODO: this is incredibly hacky lol
		let module = &module[11..(module.len() - 5)].to_string().clone();
		let file_path = format!("docs/modules/{}.md", module);

		if Path::new(file_path.as_str()).exists() {
			fs::remove_file(file_path.as_str()).expect(format!("Failed to delete {}!", file_path).as_str());
		}

		let mut file = File::create(file_path.as_str()).expect(format!("Failed to create {}!", file_path).as_str());
		file.write_all(doc.as_bytes()).expect(format!("Failed to write to {}!", file_path).as_str());
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
