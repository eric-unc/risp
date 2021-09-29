use std::io;

use crate::eval::{Environment, Val, ValList};
use crate::{check_arity_at_least, check_arity_is};

use Val::{String as StringVal, Void};

pub fn add_io_procs(env: &mut Environment) {
	env.add_proc("print".to_string(), print);
	env.add_proc("put".to_string(), put);
	env.add_proc("put-each".to_string(), put_each);
	env.add_proc("input".to_string(), input);
}

fn print(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("print", 1, rands);

	for arg in rands {
		print!("{}", arg);
	}

	Ok(Void)
}

fn put(rands: ValList) -> Result<Val, String> {
	for arg in rands {
		print!("{}", arg);
	}

	println!();

	Ok(Void)
}

fn put_each(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("put-each", 1, rands);

	for arg in rands {
		println!("{}", arg);
	}

	Ok(Void)
}

fn input(rands: ValList) -> Result<Val, String> {
	check_arity_is!("input", 0, rands);

	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	line = line.trim().to_string();

	Ok(StringVal(line))
}
