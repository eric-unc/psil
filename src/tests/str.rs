use crate::{evals_and_eq, fails_eval};
use crate::val::Val::{Boolean, Number, String};

use crate::tests::{eval, parse};

#[test]
fn to_str() {
	evals_and_eq!("(2str 5)", String("5".to_string()));
	evals_and_eq!("(2str true)", String("true".to_string()));
	evals_and_eq!("(2str false)", String("false".to_string()));
	evals_and_eq!("(2str \"string\")", String("string".to_string()));
	evals_and_eq!("(2str #sym)", String("#sym".to_string()));
	evals_and_eq!("(2str put)", String("<procedure>".to_string()));

	fails_eval!("(2str)");
	fails_eval!("(2str 5 5)");
}

#[test]
fn is_str() {
	evals_and_eq!("(is-str? 5)", Boolean(false));
	evals_and_eq!("(is-str? \"string\")", Boolean(true));

	fails_eval!("(is-str?)");
	fails_eval!("(is-str? 5 5)");
}

#[test]
fn str_cat() {
	evals_and_eq!("(str-cat 5 5)", String("55".to_string()));
	evals_and_eq!("(str-cat true \"hi\")", String("truehi".to_string()));

	fails_eval!("(str-cat)");
	fails_eval!("(str-cat 5)");
}

#[test]
fn str_contains() {
	evals_and_eq!("(str-contains? \"string\" \"ri\")", Boolean(true));
	evals_and_eq!("(str-contains? \"string\" \"qi\")", Boolean(false));

	fails_eval!("(str-contains?)");
	fails_eval!("(str-contains? \"5\")");
	fails_eval!("(str-contains? \"5\" 5)");
	fails_eval!("(str-contains? \"5\" \"5\" \"5\")");
}

#[test]
fn str_empty() {
	evals_and_eq!("(str-empty? \"\")", Boolean(true));
	evals_and_eq!("(str-empty? \"string\")", Boolean(false));

	fails_eval!("(str-empty?)");
	fails_eval!("(str-empty? \"\" \"\")");
	fails_eval!("(str-empty? 5)");
}

#[test]
fn str_insert() {
	// TODO: too lazy for this one
	fails_eval!("(str-insert)");
}

#[test]
fn str_len() {
	evals_and_eq!("(str-len \"\")", Number(0.0));
	evals_and_eq!("(str-len \"hello\")", Number(5.0));

	fails_eval!("(str-len)");
	fails_eval!("(str-len \"\" \"\")");
}

#[test]
fn str_low() {
	evals_and_eq!("(str-low \"HELLO\")", String("hello".to_string()));
	evals_and_eq!("(str-low \"HeLLo5\")", String("hello5".to_string()));

	fails_eval!("(str-low)");
	fails_eval!("(str-low \"\" \"\")");
	fails_eval!("(str-low 5)");
}

#[test]
fn str_repeat() {
	evals_and_eq!("(str-repeat \"hello\" 3)", String("hellohellohello".to_string()));

	fails_eval!("(str-repeat \"hello\" -1)");
	fails_eval!("(str-repeat \"hello\" 0.5)");

	fails_eval!("(str-repeat)");
	fails_eval!("(str-repeat \"hello\")");
	fails_eval!("(str-repeat \"hello\" true)");
}

#[test]
fn str_replace() {
	evals_and_eq!("(str-replace \"hello old\" \"old\" \"new\")", String("hello new".to_string()));

	fails_eval!("(str-replace)");
	fails_eval!("(str-replace \"hello old\")");
	fails_eval!("(str-replace \"hello old\" \"old\" )");
	fails_eval!("(str-replace \"hello old\" \"old\" 5)");
}

#[test]
fn str_starts_with() {
	evals_and_eq!("(str-starts-with? \"helloyes\" \"hello\")", Boolean(true));
	evals_and_eq!("(str-starts-with? \"helloyes\" \"yes\")", Boolean(false));

	fails_eval!("(str-starts-with?)");
	fails_eval!("(str-starts-with? \"helloyes\")");
	fails_eval!("(str-starts-with? \"helloyes\" 5)");
}

#[test]
fn str_strip() {
	evals_and_eq!("(str-strip \"    helloyes \")", String("helloyes".to_string()));
	evals_and_eq!("(str-strip \"helloyes\")", String("helloyes".to_string()));

	fails_eval!("(str-strip)");
	fails_eval!("(str-strip 5)");
}

#[test]
fn str_trunc() {
	evals_and_eq!("(str-trunc \"hello\" 3)", String("hel".to_string()));

	fails_eval!("(str-trunc)");
	fails_eval!("(str-trunc \"hello\")");
	fails_eval!("(str-trunc \"hello\" \"hello\")");
	fails_eval!("(str-trunc \"hello\" -1)");
}

#[test]
fn str_up() {
	evals_and_eq!("(str-up \"hello\")", String("HELLO".to_string()));
	evals_and_eq!("(str-up \"HeLLo5\")", String("HELLO5".to_string()));

	fails_eval!("(str-up)");
	fails_eval!("(str-up \"\" \"\")");
	fails_eval!("(str-up 5)");
}
