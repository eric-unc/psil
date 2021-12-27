mod control;
mod io;
mod list;
mod str;
mod math;
mod boolean;
mod symb;
mod proc;

use crate::environment::Environment;

/// Actually extends environment with natively-defined functions
pub fn add_native_library(env: &mut Environment) {
	control::add_control_procs(env);
	io::add_io_procs(env);
	list::add_list_procs(env);
	str::add_str_procs(env);
	math::add_math_procs(env);
	boolean::add_boolean_procs(env);
	symb::add_symbol_procs(env);
	proc::add_procedure_procs(env);
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
macro_rules! fail_on_bad_type {
	( $proc_name:literal, $expected_type:literal, $rands:expr ) => {
		return Err(format!("Native proc '{}' expected a {}!", $proc_name, $expected_type))
	}
}

#[macro_export]
macro_rules! get_list {
	( $proc_name:literal, $rands:expr, $index:literal ) => {
		match &$rands[$index] {
			List(l) => l,
			_ => fail_on_bad_type!($proc_name, "list", $rands)
		}
	}
}
