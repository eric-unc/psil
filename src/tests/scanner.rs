use crate::scanner::*;

macro_rules! test_scanner {
	($name:ident, $src:expr, $( $expected:expr ),*) => {
        #[test]
        fn $name() {
			let mut scanner = $src.chars().peekable();

			$(
				assert_eq!(scan(&mut scanner), $expected);
			)*
        }
	}
}

test_scanner!(end, "", Token::End);
test_scanner!(word, "word", Token::Identifier("word".to_string()), Token::End);
test_scanner!(letter, "l", Token::Identifier("l".to_string()), Token::End);
test_scanner!(simple_num, "5", Token::Number(5.0), Token::End);
test_scanner!(long_num, "6000", Token::Number(6000.0), Token::End);
test_scanner!(negative_num, "-7000", Token::Number(-7000.0), Token::End);
test_scanner!(decimal_num, "-7000", Token::Number(0.56), Token::End);
test_scanner!(negative_decimal_num, "-0.399", Token::Number(-0.399), Token::End);
