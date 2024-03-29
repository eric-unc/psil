use crate::{evals_and_eq, fails_eval};
use crate::val::Val::{Number, List, Boolean, StringV};
use crate::tests::{eval, parse};

#[test]
fn is_list() {
	evals_and_eq!("(is-list? (list 5))", Boolean(true));
	evals_and_eq!("(is-list? is-list?)", Boolean(false));
	evals_and_eq!("(is-list? 5)", Boolean(false));

	fails_eval!("(is-list?)");
	fails_eval!("(is-list? (list) (list))");
}

#[test]
fn list() {
	evals_and_eq!("(list)", List(vec![]));
	evals_and_eq!("(list 1)", List(vec![Number(1.0)]));
	evals_and_eq!("(list 1 false \"pee\")", List(vec![Number(1.0), Boolean(false), StringV("pee".to_string())]));
}

#[test]
fn list_append() {
	evals_and_eq!("(list-append (list) 1)", List(vec![Number(1.0)]));
	evals_and_eq!("(list-append (list false) 1)", List(vec![Boolean(false), Number(1.0)]));
	evals_and_eq!("(list-append (list false) 1 2)", List(vec![Boolean(false), Number(1.0), Number(2.0)]));
	fails_eval!("(list-append 1 3)");
}

#[test]
fn list_count() {
	evals_and_eq!("(list-count (list) 1)", Number(0.0));
	evals_and_eq!("(list-count (list 1 2 3 1) 1)", Number(2.0));
	evals_and_eq!("(list-count (list 1 2 3 1) 1 3)", Number(3.0));
	fails_eval!("(list-count 1 3)");
	fails_eval!("(list-count (list))");
}

#[test]
fn list_empty() {
	evals_and_eq!("(list-empty? (list 5))", Boolean(false));
	evals_and_eq!("(list-empty? (list))", Boolean(true));
	evals_and_eq!("(list-empty? (list 1 43 #void))", Boolean(false));

	fails_eval!("(list-empty?)");
	fails_eval!("(list-empty? (list) (list))");
	fails_eval!("(list-empty? 5)");
}

#[test]
fn list_filter() {
	evals_and_eq!("(list-filter (list 1 false \"pee\") is-num?)", List(vec![Number(1.0)]));
	evals_and_eq!("(list-filter (list 1 2 3 4 5 6) {|n| (== (% n 2) 0)})", List(vec![Number(2.0), Number(4.0), Number(6.0)]));
	fails_eval!("(list-filter (list 1 false \"pee\"))");
	fails_eval!("(list-filter (list 1 false \"pee\") is-num? is-num?)");
	fails_eval!("(list-filter (list 1 5 6) put)");
}

#[test]
fn list_filter_not() {
	evals_and_eq!("(list-filter-not (list 1 false \"pee\") is-num?)", List(vec![Boolean(false), StringV("pee".to_string())]));
	fails_eval!("(list-filter-not (list 1 false \"pee\"))");
	fails_eval!("(list-filter-not (list 1 false \"pee\") is-num? is-num?)");
	fails_eval!("(list-filter-not (list 1 5 6) put)");
}

#[test]
fn list_find() {
	evals_and_eq!("(list-find (list 1 2 3) 2)", Number(1.0));
	evals_and_eq!("(list-find (list 1 2 3 2) 2)", Number(1.0));
	evals_and_eq!("(list-find (list 0 0 3 2) 3)", Number(2.0));
	evals_and_eq!("(list-find (list 0 0 3 2) 5)", Number(-1.0));
	evals_and_eq!("(list-find (list 0 0 3 2) \"0\")", Number(-1.0));
	fails_eval!("(list-find (list 0 0 3 2) 3 5)");
	fails_eval!("(list-find 3 (list 0 0 3 2))");
}

#[test]
fn list_flatten() {
	evals_and_eq!("(list-flatten (list 1 2 (list 3 4) 5))", List(vec![Number(1.0), Number(2.0), Number(3.0), Number(4.0), Number(5.0)]));
	evals_and_eq!("(list-flatten (list 1 2 (list 3 (list 4 5)) 6))", List(vec![Number(1.0), Number(2.0), Number(3.0), List(vec![Number(4.0), Number(5.0)]), Number(6.0)]));
	evals_and_eq!("(list-flatten (list 1 2 (list 3 (list 4 5)) 6) 2)", List(vec![Number(1.0), Number(2.0), Number(3.0), Number(4.0), Number(5.0), Number(6.0)]));
	fails_eval!("(list-flatten)");
	fails_eval!("(list-flatten (list 1 2 (list 3 4) 5) 1 1)");
	fails_eval!("(list-flatten 1 (list 1 2 (list 3 4) 5))");
}

#[test]
fn list_fold() {
	evals_and_eq!("(list-fold (list 1 2 3 4 5) + 0)", Number(15.0));
	evals_and_eq!("(list-fold (list 1 2 3 4 5 6) list-append (list))", List(vec![Number(1.0), Number(2.0), Number(3.0), Number(4.0), Number(5.0), Number(6.0)]));
	fails_eval!("(list-fold)");
	fails_eval!("(list-fold (list 1 2 3 4 5))");
	fails_eval!("(list-fold (list 1 2 3 4 5) +)");
}

#[test]
fn list_foldr() {
	evals_and_eq!("(list-foldr (list 1 2 3 4 5) + 0)", Number(15.0));
	fails_eval!("(list-foldr)");
	fails_eval!("(list-foldr (list 1 2 3 4 5))");
	fails_eval!("(list-foldr (list 1 2 3 4 5) +)");
}

#[test]
fn list_get() {
	evals_and_eq!("(list-get (list 1 false \"pee\") 0)", Number(1.0));
	evals_and_eq!("(list-get (list 1 false \"pee\") 1)", Boolean(false));
	evals_and_eq!("(list-get (list 1 false \"pee\") 2)", StringV("pee".to_string()));
	fails_eval!("(list-get (list 1 false \"pee\") 3)");
	fails_eval!("(list-get 1 3)");
	fails_eval!("(list-get (list 1 false \"pee\") 2 2)");
}

#[test]
fn list_has() {
	evals_and_eq!("(list-has? (list 1 false \"pee\") 0)", Boolean(false));
	evals_and_eq!("(list-has? (list 1 false \"pee\") 1)", Boolean(true));
	evals_and_eq!("(list-has? (list 4 false \"pee\" true 0 1 1) 1)", Boolean(true));
	fails_eval!("(list-has?)");
	fails_eval!("(list-has? (list 1 false \"pee\"))");
	fails_eval!("(list-has? 0 (list 1 false \"pee\"))");
}

#[test]
fn list_join() {
	evals_and_eq!("(list-join (list) (list 1))", List(vec![Number(1.0)]));
	evals_and_eq!("(list-join (list false) (list 1 2))", List(vec![Boolean(false), Number(1.0), Number(2.0)]));
	fails_eval!("(list-join)");
	fails_eval!("(list-join (list 1))");
	fails_eval!("(list-join (list 1) 3)");
}

#[test]
fn list_len() {
	evals_and_eq!("(list-len (list))", Number(0.0));
	evals_and_eq!("(list-len (list 5))", Number(1.0));
	evals_and_eq!("(list-len (list 9 0))", Number(2.0));
	fails_eval!("(list-len 5)");
	fails_eval!("(list-len (list 9 0) (list 9 0))");
}

#[test]
fn list_map() {
	evals_and_eq!("(list-map (list) {|n| (+ n 3)})", List(vec![]));
	evals_and_eq!("(list-map (list 12 22) {|n| (+ n 3)})", List(vec![Number(15.0), Number(25.0)]));
	fails_eval!("(list-map (list 12 22))");
	fails_eval!("(list-map (list 12 22) +)");
	fails_eval!("(list-map 22 {|n| (+ n 3)})");
}

#[test]
fn list_range() {
	evals_and_eq!("(list-range 1 2)", List(vec![Number(1.0), Number(2.0)]));
	evals_and_eq!("(list-range 2 5)", List(vec![Number(2.0), Number(3.0), Number(4.0), Number(5.0)]));
	evals_and_eq!("(list-range 2 8 2)", List(vec![Number(2.0), Number(4.0), Number(6.0), Number(8.0)]));
	fails_eval!("(list-range)");
	fails_eval!("(list-range 1)");
	fails_eval!("(list-range 1 1 1 1)");
}

#[test]
fn list_reverse() {
	evals_and_eq!("(list-reverse (list))", List(vec![]));
	evals_and_eq!("(list-reverse (list 1 false \"pee\"))", List(vec![StringV("pee".to_string()), Boolean(false), Number(1.0)]));
	fails_eval!("(list-reverse (list) (list))");
	fails_eval!("(list-reverse 1)");
}

#[test]
fn list_remove() {
	evals_and_eq!("(list-remove (list 1 2 3) 0)", List(vec![Number(2.0), Number(3.0)]));
	evals_and_eq!("(list-remove (list 1 2 3) 1)", List(vec![Number(1.0), Number(3.0)]));
	evals_and_eq!("(list-remove (list 1 2 3) 2)", List(vec![Number(1.0), Number(2.0)]));
	fails_eval!("(list-remove (list 1 2 3) 3)");
	fails_eval!("(list-remove (list 1 2 3))");
	fails_eval!("(list-remove 1)");
}

#[test]
fn list_set() {
	evals_and_eq!("(list-set (list 1 2 3) 0 3)", List(vec![Number(3.0), Number(2.0), Number(3.0)]));
	evals_and_eq!("(list-set (list 1 true 3) 1 5)", List(vec![Number(1.0), Number(5.0), Number(3.0)]));
	fails_eval!("(list-set)");
	fails_eval!("(list-set (list 1 2 3) -1 3)");
	fails_eval!("(list-set (list 1 2 3) 3 3)");
}

#[test]
fn list_swap() {
	evals_and_eq!("(list-swap (list 1 2 3) 0 1)", List(vec![Number(2.0), Number(1.0), Number(3.0)]));
	evals_and_eq!("(list-swap (list 1 2 3 4) 1 2)", List(vec![Number(1.0), Number(3.0), Number(2.0), Number(4.0)]));
	fails_eval!("(list-swap)");
	fails_eval!("(list-swap (list 1 2 3 4) 1 5)");
}
