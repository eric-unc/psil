mod boolean;
mod control;
mod io;
mod list;
mod math;
mod proc;
mod str;
mod symb;
mod table;

use crate::environment::Environment;

pub fn add_standard_library(env: &mut Environment) {
	add_native_library(env);
	add_pure_library(env);
}


fn add_native_library(env: &mut Environment) {
	boolean::add_native(env);
	control::add_native(env);
	io::add_native(env);
	list::add_native(env);
	math::add_native(env);
	proc::add_native(env);
	str::add_native(env);
	symb::add_native(env);
	table::add_native(env);
}

fn add_pure_library(env: &mut Environment) {
	control::add_pure(env);
	list::add_pure(env);
}

// Macros for module

#[macro_export]
macro_rules! check_arity_at_least {
	( $proc_name:literal, $arity:literal, $rands:expr ) => {
		if $rands.len() < $arity {
			if $arity == 1 {
				return Err(format!("Native proc '{}' expected at least {} rand! Given {}.", $proc_name, $arity, $rands.len()))
			} else {
				return Err(format!("Native proc '{}' expected at least {} rands! Given {}.", $proc_name, $arity, $rands.len()))
			}
		}
	}
}

#[macro_export]
macro_rules! check_arity_is {
	( $proc_name:literal, $arity:literal, $rands:expr ) => {
		if $rands.len() != $arity {
			return match $arity {
				0 => Err(format!("Native proc '{}' expected no rands! Given {}.", $proc_name, $rands.len())),
				1 => Err(format!("Native proc '{}' expected 1 rand! Given {}.", $proc_name, $rands.len())),
				_ => Err(format!("Native proc '{}' expected {} rands! Given {}.", $proc_name, $arity, $rands.len()))
			}
		}
	}
}

/// inclusively between
#[macro_export]
macro_rules! check_arity_between {
	( $proc_name:literal, $low_arity:literal, $high_arity:literal, $rands:expr ) => {
		// So basically, the line after returns a warning when low_arity is 0.
		// So we want to ignore that, so 0..1 is accepted
		// TODO: if I was really good at macros, I probably wouldn't need to do this.
		#[allow(unused_comparisons)]
		if $rands.len() < $low_arity || $rands.len() > $high_arity {
			return Err(format!("Native proc '{}' expected {} to {} rands! Given {}.", $proc_name, $low_arity, $high_arity, $rands.len()))
		}
	}
}

#[macro_export]
macro_rules! check_arity_even {
	( $proc_name:literal, $rands:expr ) => {
		if $rands.len() % 2 != 0 {
			return Err(format!("Native proc '{}' expected an even number of rands! Given {}.", $proc_name, $rands.len()));
		}
	}
}

#[macro_export]
macro_rules! fail_on_bad_type {
	( $proc_name:literal, $expected_type:literal, $rands:expr ) => {
		return Err(format!("Native proc '{}' expected a {}!", $proc_name, $expected_type))
	}
}

#[macro_export]
macro_rules! get_number {
	( $proc_name:literal, $rands:expr, $index:expr ) => {
		match &$rands[$index] {
			Number(n) => *n,
			_ => fail_on_bad_type!($proc_name, "number", $rands)
		}
	}
}

#[macro_export]
macro_rules! get_integer {
	( $proc_name:literal, $rands:expr, $index:expr ) => {
		match &$rands[$index] {
			Number(n) => {
				if *n % 1.0 != 0.0 {
					return Err(format!("Proc '{}' expected an integer!", $proc_name));
				} else {
					*n as i64
				}
			}
			_ => fail_on_bad_type!($proc_name, "number", $rands)
		}
	}
}

#[macro_export]
macro_rules! get_natural_number {
	( $proc_name:literal, $rands:expr, $index:expr ) => {
		match &$rands[$index] {
			Number(n) => {
				if *n < 0.0 || *n % 1.0 != 0.0 {
					return Err(format!("Proc '{}' expected a natural number!", $proc_name));
				} else {
					*n as u64
				}
			}
			_ => fail_on_bad_type!($proc_name, "number", $rands)
		}
	}
}

#[macro_export]
macro_rules! get_boolean {
	( $proc_name:literal, $rands:expr, $index:expr ) => {
		match &$rands[$index] {
			Boolean(b) => b,
			_ => fail_on_bad_type!($proc_name, "boolean", $rands)
		}
	}
}

#[macro_export]
macro_rules! get_string {
	( $proc_name:literal, $rands:expr, $index:expr ) => {
		match &$rands[$index] {
			StringV(s) => s,
			_ => fail_on_bad_type!($proc_name, "string", $rands)
		}
	}
}

#[macro_export]
macro_rules! get_symbol {
	( $proc_name:literal, $rands:expr, $index:expr ) => {
		match &$rands[$index] {
			Symbol(s) => s,
			_ => fail_on_bad_type!($proc_name, "symbol", $rands)
		}
	}
}

#[macro_export]
macro_rules! get_proc {
	( $proc_name:literal, $rands:expr, $index:expr ) => {
		match &$rands[$index] {
			ProcedureV(p) => p,
			_ => fail_on_bad_type!($proc_name, "procedure", $rands)
		}
	}
}

#[macro_export]
macro_rules! get_list {
	( $proc_name:literal, $rands:expr, $index:expr ) => {
		match &$rands[$index] {
			List(l) => l,
			_ => fail_on_bad_type!($proc_name, "list", $rands)
		}
	}
}
