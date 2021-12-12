use crate::scanner::*;
use crate::scanner::Token::*;

macro_rules! test_scanner {
	($name:ident, $src:expr, $( $expected:expr ),*) => {
        #[test]
        fn $name() {
			let mut scanner = $src.chars().peekable();

			$(
				let token = scan(&mut scanner);
				assert!(token.is_ok(), "{} does not lex!", $src);
				assert_eq!(token.unwrap(), $expected);
			)*
        }
	}
}

// Identifiers
test_scanner!(end, "", End);
test_scanner!(word, "word", Identifier("word".to_string()), End);
test_scanner!(letter, "l", Identifier("l".to_string()), End);
test_scanner!(identifier_with_numbers, "bla555", Identifier("bla555".to_string()), End);
test_scanner!(identifier_starting_with_numbers, "555bla", Identifier("555bla".to_string()), End);

// Symbols
test_scanner!(left_paren, "(", LeftParen, End);
test_scanner!(right_paren, ")", RightParen, End);
test_scanner!(left_bracket, "{", LeftBracket, End);
test_scanner!(right_bracket, "}", RightBracket, End);
test_scanner!(bar, "|", Bar, End);

// Numbers
test_scanner!(simple_num, "5", Number(5.0), End);
test_scanner!(long_num, "6000", Number(6000.0), End);
test_scanner!(negative_num, "-7000", Number(-7000.0), End);
test_scanner!(decimal_num, "0.56", Number(0.56), End);
test_scanner!(negative_decimal_num, "-0.399", Number(-0.399), End);
test_scanner!(num_starting_with_dot, ".56", Number(0.56), End);

// Strings
test_scanner!(string, "\"I like Pizza!\"", String("I like Pizza!".to_string()), End);
test_scanner!(string_escapes, "\"I...\\\\ \\n \\r \\t \\\" \"", String("I...\\ \n \r \t \" ".to_string()), End);

#[test]
fn incomplete_string() {
	let mut scanner = "\"I like".chars().peekable();
	let token = scan(&mut scanner);
	assert!(token.is_err());
	assert_eq!(token.unwrap_err(), ScannerError::IncompleteString);
}

#[test]
fn unknown_escape_char() {
	let mut scanner = "\" \\z \"".chars().peekable();
	let token = scan(&mut scanner);
	assert!(token.is_err());
	assert_eq!(token.unwrap_err(), ScannerError::UnknownEscapeChar('z'));
}

#[test]
fn incomplete_string_through_escape_char() {
	let mut scanner = "\"I like \\".chars().peekable();
	let token = scan(&mut scanner);
	assert!(token.is_err());
	assert_eq!(token.unwrap_err(), ScannerError::IncompleteString);
}

// Other literals
test_scanner!(symbol, "#symbol", Symbol("symbol".to_string()), End);
test_scanner!(true_lit, "true", Boolean(true), End);
test_scanner!(false_lit, "false", Boolean(false), End);

// Keywords
test_scanner!(if_form, "if", If, End);
test_scanner!(cond, "cond", Cond, End);
test_scanner!(define, "define", Define, End);
test_scanner!(do_form, "do", Do, End);
test_scanner!(and, "and", And, End);
test_scanner!(or, "or", Or, End);

// Spacing
test_scanner!(spaces, "    ", End);
test_scanner!(whitespace_numbers, "  \r\t\n 9  \t \n \r 10", Number(9.0), Number(10.0), End);

// Composite tokens
test_scanner!(put, "(put)", LeftParen, Identifier("put".to_string()), RightParen, End);
test_scanner!(hello_world, "(put \"Hello World\")", LeftParen, Identifier("put".to_string()), String("Hello World".to_string()), RightParen, End);
test_scanner!(sin_approx, "(define sin {|x| x})", LeftParen, Define, Identifier("sin".to_string()), LeftBracket, Bar, Identifier("x".to_string()), Bar, Identifier("x".to_string()), RightBracket, RightParen, End);
