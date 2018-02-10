#![feature(conservative_impl_trait)]
#![feature(trace_macros)]
#![feature(fn_traits)]

//trace_macros!(true);

mod lexer;

use std::boxed::Box;
use std::fmt::Debug;

pub trait Statement: Debug {
    fn to_string(&self) -> String;
    fn into_token(&self) -> Option<Token> { None }
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

impl Statement for Number {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token(char);

impl Statement for Token {
    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn into_token(&self) -> Option<Token> { 
        Some(Token(self.0))
    }
}

macro_rules! rule {
    ($name: ident, or[$($parse_funcs: expr), +]) => {
        fn $name(lexer: &mut lexer::Lexer) -> Option<Box<Statement>> {
            $(
                let result = $parse_funcs(lexer);

                if result.is_some() {
                    lexer.accept();
                    return result
                } else {
                    lexer.reject();
                }
            )+;

            None
        }
    };
    ($name: ident, and[$($parse_funcs: expr), +] => $nandler_func: expr) => {
        fn $name(lexer: &mut lexer::Lexer) -> Option<Box<Statement>> {
            let results = ($(match $parse_funcs(lexer) {
                Some(statement) => statement,
                _ => return None
            }), +);
            
            std::ops::Fn::call(&$nandler_func, results)
        }
    };
    ($name: ident, $parse_func:expr) => {
        rule!($name, or[$parse_func])
    };
}

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

fn token(token_char: char) -> impl FnMut(&mut lexer::Lexer) -> Option<Box<Statement>> {
    move |ref mut lexer| {
        lexer
        .next()
        .map(|c| c as char).and_then(|c| 
            if c == token_char {
                Some(Box::new(Token(c)) as Box<Statement>) 
            } else {
                None
            })
    }
}

fn sum_expr(left_num: Box<Statement>, sign: Box<Statement>, right_num: Box<Statement>) -> Option<Box<Statement>> {
    match sign.into_token() {
        Some(Token('+')) => Some(Box::new(OperatorStatement::Plus(left_num, right_num))),
        Some(Token('-')) => Some(Box::new(OperatorStatement::Minus(left_num, right_num))),
        Some(Token('/')) => Some(Box::new(OperatorStatement::Div(left_num, right_num))),
        Some(Token('*')) => Some(Box::new(OperatorStatement::Mul(left_num, right_num))),
        _ => panic!("Expected token from parser. Got: {:?}", sign)
    }
    
}

// expr = term, expr1;
// expr1 = "+",term,expr1 | "-",term,expr1;
// term = factor, term1;
// term1 = "*", factor, term1 | "/", factor, term1;
// factor = "(", expr , ")" | number;
// syntax = expr;

// const STRING1: &str = "1 + 2";
// const STRING2: &str = "(1 + 2)";
// const STRING3: &str = "(1 + 2) + 3";
// const STRING4: &str = "1 + (2 + 3)";
// const STRING5: &str = "(1 + 2) + (3 + 4)";
const STRING6: &str = "((1 + 2) + (3 + 4))";

fn main() {
    let ref mut lex = lexer::Lexer::new(STRING6);

    rule!(space, token(' '));
    rule!(lbrace, token('('));
    rule!(plus, token('+'));
    rule!(test, or[token('+'), token('(')]);
    rule!(test_and, and[parse_num, token('+'), parse_num] => sum_expr);

    println!("ZPT0: {:?}", space(lex));
    println!("ZPT1: {:?}", lbrace(lex));
    println!("ZPT2: {:?}", plus(lex));
    println!("ZPT3: {:?}", test(lex));
    println!("ZPT3: {:?}", test_and(lex));
}
