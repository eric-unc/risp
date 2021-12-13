use std::iter::Peekable;
use std::str::Chars;

use crate::ast::*;
use crate::scanner::*;
use crate::scanner::Token::*;

pub enum ParserError {
	PScannerError(ScannerError),
	UnexpectedToken(Token)
}
use ParserError::*;

impl From<ScannerError> for ParserError {
	fn from(e: ScannerError) -> Self {
		ParserError::PScannerError(e)
	}
}

pub fn parse(str: String) -> Result<ProgramAst, ParserError> {
	let mut scanner = Scanner::new(str.as_str());
	parse_program(&mut scanner)
}

// program ::= expr_list?
pub fn parse_program(scanner: &mut Scanner) -> Result<ProgramAst, ParserError> {
	let mut list = vec![];

	if scanner.peek()? != Token::End {
		list = parse_expr_list(scanner)?;
	}

	Ok(ProgramAst { expr_list: list })
}

// expr_list ::= expr+
fn parse_expr_list(scanner: &mut Scanner) -> Result<ExprListAst, ParserError> {
	let mut list = vec![];

	list.push(parse_expr(scanner)?);

	while is_expr_start(scanner.peek()?) {
		list.push(parse_expr(scanner)?);
	}

	Ok(list)
}

fn is_expr_start(token: Token) -> bool {
	/*token == Identifier(_) || token == LeftParen || token == LeftBracket || token == Number(_) || token == Boolean(_) || token == StringT(_) || token == Symbol(_) || token == If || token == Cond || token == Define || token == Do || token == And || token == Or*/
	match token {
		Identifier(_) | LeftParen | LeftBracket | Number(_) | Boolean(_) | StringT(_) | Symbol(_) | If | Cond | Define | Do | And | Or => true,
		_ => false
	}
}

// expr ::= atom | invocation
pub fn parse_expr(scanner: &mut Scanner) -> Result<ExprAst, ParserError> {
	if scanner.peek()? != LeftParen {
		Ok(ExprAst::Atom(Box::from(parse_atom(scanner)?)))
	} else {
		Ok(ExprAst::NewInvocation(parse_invocation(scanner)?))
	}
}

// atom ::= number | boolean | string | lambda | name
fn parse_atom(scanner: &mut Scanner) -> Result<AtomAst, ParserError> {
	match scanner.peek()? {
		Number(n) => Ok(AtomAst::Number(n)),
		Boolean(b) => Ok(AtomAst::Boolean(b)),
		StringT(s) => Ok(AtomAst::String(s)),
		LeftBracket => Ok(AtomAst::Lambda(parse_lambda(scanner)?)),
		o => Err(UnexpectedToken(o))
	}
}

// invocation ::= ( identifier expr_list? )
fn parse_invocation(scanner: &mut Scanner) -> Result<NewInvocationAst, ParserError> {
	expect(scanner, LeftParen)?;

	let proc = match scanner.scan()? {
		Identifier(i) => PossibleProc::Name(i),
		If => PossibleProc::SpecialForm(SpecialForms::If),
		Cond => PossibleProc::SpecialForm(SpecialForms::Cond),
		Define => PossibleProc::SpecialForm(SpecialForms::Define),
		Do => PossibleProc::SpecialForm(SpecialForms::Do),
		And => PossibleProc::SpecialForm(SpecialForms::And),
		Or => PossibleProc::SpecialForm(SpecialForms::Or),
		u => return Err(ParserError::UnexpectedToken(u))
	};

	if scanner.peek()? == RightParen {
		scanner.scan()?;
		Ok(NewInvocationAst { proc, expr_list: vec![] })
	} else {
		let expr_list = parse_expr_list(scanner)?;
		expect(scanner, RightParen)?;
		Ok(NewInvocationAst { proc, expr_list })
	}
}

// lambda ::= { params? expr }
fn parse_lambda(scanner: &mut Scanner) -> Result<LambdaAst, ParserError> {
	expect(scanner, LeftBracket)?;

	let params =
		if scanner.peek()? == Bar {
			parse_params(scanner)?
		} else {
			ParamsAst { names: vec![] }
		};

	let expr = parse_expr(scanner)?;

	expect(scanner, RightBracket)?;

	Ok(LambdaAst { params, expr })
}

// params ::= | identifier+ |
fn parse_params(scanner: &mut Scanner) -> Result<ParamsAst, ParserError> {
	expect(scanner, Bar)?;

	let mut names = vec![];

	while scanner.peek()? != Bar {
		names.push(expect_identifier(scanner)?);
	}

	expect(scanner, Bar)?;

	Ok(ParamsAst { names })
}

fn expect(scanner: &mut Scanner, token: Token) -> Result<Token, ParserError> {
	match scanner.scan() {
		Ok(t) =>
			if t == token {
				Ok(t)
			} else {
				Err(UnexpectedToken(t))
			}
		Err(e) => Err(PScannerError(e))
	}
}

fn expect_identifier(scanner: &mut Scanner) -> Result<String, ParserError> {
	match scanner.scan() {
		Ok(Identifier(i)) => Ok(i),
		Ok(t) => Err(UnexpectedToken(t)),
		Err(e) => Err(PScannerError(e))
	}
}
