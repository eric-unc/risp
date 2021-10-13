use crate::{evals_and_eq, evals_and_eq_with_env, fails_eval};
use crate::val::Val::{Boolean, Number, Symbol};
use crate::val::void;

use crate::environment::Environment;
use crate::tests::{eval, eval_with_env, parse};

// TODO: kind of a pain to test these properly

#[test]
fn put() {
	evals_and_eq!("(put)", void());
	evals_and_eq!("(put \"Hello world!\")", void());
}

#[test]
fn put_each() {
	fails_eval!("(put-each)");
	evals_and_eq!("(put-each \"Hello\" \"world!\")", void());
}

#[test]
fn print() {
	fails_eval!("(print)");
	evals_and_eq!("(print \"Hello\" \"world!\")", void());
}

#[test]
fn input() {
	fails_eval!("(input 1)");
}
