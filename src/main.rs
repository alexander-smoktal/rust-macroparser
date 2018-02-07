#![feature(trace_macros)]

trace_macros!(true);

mod lexer;

use std::boxed::Box;
use std::fmt::Debug;

pub trait Statement: Debug {
    fn to_string(&self) -> String;
}

#[derive(Debug)]
enum OperatorStatement {
    Plus(Box<Statement>, Box<Statement>),
    Minus(Box<Statement>, Box<Statement>),
    Mul(Box<Statement>, Box<Statement>),
    Div(Box<Statement>, Box<Statement>)
}

impl Statement for OperatorStatement {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
struct Number {
    num: f32
}

impl Number {
    pub fn new(num: f32) -> Self {
        Number {
            num
        }
    }
}

#[derive(Debug)]
struct Noop;

impl Statement for Noop {
    fn to_string(&self) -> String {
        "Noop".to_string()
    }
}

impl Statement for Number {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}


macro_rules! rule {
    ($lexer:ident, or[]) => { Option::None::<Box<Statement>> };
    ($lexer:ident, or[$parse_func:expr, $($parse_funcs:expr), *]) => {{
        let result = $parse_func($lexer);

        if result.is_some() {
            $lexer.accept();

            result
        } else {
            $lexer.reject();
            
            rule!($lexer, $($parse_funcs), *) 
        }}
    };
    ($lexer:ident, $token:expr) => {
        $lexer.next().map(|c| c as char).and_then(|c| if c == $token { Some(Box::new(Noop) as Box<Statement>) } else { None })
    };
}

// expr = term, expr1;
// expr1 = "+",term,expr1 | "-",term,expr1;
// term = factor, term1;
// term1 = "*", factor, term1 | "/", factor, term1;
// factor = "(", expr , ")" | number;
// syntax = expr;

fn parse_num(lexer: &mut lexer::Lexer) -> Option<Box<Statement>> {
    let result = lexer
        .next()
        .map(|c| c as char)
        .and_then(|c|
                  if c.is_numeric() {
                      Some(Box::new(Number::new(c.to_string().parse::<f32>().unwrap())) as Box<Statement>)
                  } else {
                      None
                  });

    result
}


const STRING1: &str = "1 + 2";
const STRING2: &str = "(1 + 2)";
const STRING3: &str = "(1 + 2) + 3";
const STRING4: &str = "1 + (2 + 3)";
const STRING5: &str = "(1 + 2) + (3 + 4)";
const STRING6: &str = "((1 + 2) + (3 + 4))";

fn main() {
    let mut lex = lexer::Lexer::new(STRING6);

    println!("ZPT0: {:?}", rule!(lex, ' '));
    println!("ZPT1: {:?}", rule!(lex, '('));
    println!("ZPT2: {:?}", rule!(lex, or[]));
    println!("ZPT2: {:?}", rule!(lex, or[' ', '(']));

    rule!(lex, or[' ', '('])
}
