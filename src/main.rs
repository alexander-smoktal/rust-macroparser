#![feature(conservative_impl_trait)]
#![feature(trace_macros)]
#![feature(fn_traits)]

//trace_macros!(true);

mod lexer;

use std::boxed::Box;

#[derive(Debug)]
pub enum Statement {
    Operator {
        op: Box<Statement>,
        left: Box<Statement>,
        right: Box<Statement>
    },
    Number(f32),
    Token(char),
    StatList(Vec<Box<Statement>>)
}

macro_rules! or {
    [$($parse_funcs: expr),+] => {
        |lexer: &mut lexer::Lexer| -> Option<Box<Statement>> {
            $(
                let result = $parse_funcs(lexer);

                if result.is_some() {
                    lexer.accept();
                    return result
                } else {
                    lexer.reject()
                }
            )+;

            None
        }
    }
}

macro_rules! and {
    [($($parse_funcs: expr),+) => $nandler_func: expr] => {
        |lexer: &mut lexer::Lexer| -> Option<Box<Statement>> {
            let results = ($(match $parse_funcs(lexer) {
                Some(statement) => statement,
                _ => {
                    lexer.reject();
                    return None
                }
            }), +);

            match std::ops::Fn::call(&$nandler_func, results) {
                statement @ Some(_) => {
                    lexer.accept();
                    statement
                }
                _ => {
                    lexer.reject();
                    return None
                }
            }
        }
    };
}

macro_rules! rule {
    ($name: ident, $parse_func:expr) => {
        fn $name(lexer: &mut lexer::Lexer) -> Option<Box<Statement>> {
            $parse_func(lexer)
        }
    };
}

fn num(lexer: &mut lexer::Lexer) -> Option<Box<Statement>> {
    let result = lexer
        .next()
        .map(|c| c as char)
        .and_then(|c|
                  if c.is_numeric() {
                      lexer.accept();
                      Some(Box::new(Statement::Number(c.to_string().parse::<f32>().unwrap())))
                  } else {
                      lexer.reject();
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
                lexer.accept();
                Some(Box::new(Statement::Token(c)))
            } else {
                lexer.reject();
                None
            })
    }
}

fn rearrange_operator(lstatement: Box<Statement>, rstatements: Box<Statement>) -> Option<Box<Statement>> {
    match *rstatements {
        Statement::StatList(mut list) => Some(Box::new(Statement::StatList(vec![
                Box::new(Statement::Operator{
                    op: list.pop().unwrap(),
                    left: lstatement,
                    right: list.pop().unwrap()
                }),
                list.pop().unwrap()
            ]))),
        _ => panic!("Got not a statement list in a rule")
    }
}

fn make_statlist(stat1: Box<Statement>, stat2: Box<Statement>, stat3: Box<Statement>) -> Option<Box<Statement>> {
     Some(Box::new(Statement::StatList(vec![stat1, stat2, stat3])))
}

// expr = term, expr1;
// expr1 = "+",term,expr1 | "-",term,expr1;
// term = factor, term1;
// term1 = "*", factor, term1 | "/", factor, term1;
// factor = "(", expr , ")" | number;
// syntax = expr;

rule!(expr, and![(term, expr1) => rearrange_operator]);

rule!(expr1, or![
     and![(token('+'), term, expr1) => make_statlist],
     and![(token('-'), term, expr1) => make_statlist]
]);

rule!(term, and![(factor, term1) => rearrange_operator]);

rule!(term1, or![
    and![(token('*'), factor, term1) => make_statlist],
    and![(token('/'), factor, term1) => make_statlist]
]);

rule!(factor, or![
    and![(token('('), expr, token(')')) => |_lbrace, stat, _rbrace| Some(stat)],
    num
]);

rule!(syntax, expr);

const STRING1: &str = "1 + 2";
const STRING2: &str = "(1 + 2)";
const STRING3: &str = "(1 + 2) * 3";
const STRING4: &str = "1 + (2 + 3)";
const STRING5: &str = "(1 * 2) + (3 * 4)";
const STRING6: &str = "((1 * 2) + (3 + 4))";

fn main() {
    println!("0. Result {:?}", syntax(&mut lexer::Lexer::new(STRING1)));
    println!("1. Result {:?}", syntax(&mut lexer::Lexer::new(STRING2)));
    println!("2. Result {:?}", syntax(&mut lexer::Lexer::new(STRING3)));
    println!("3. Result {:?}", syntax(&mut lexer::Lexer::new(STRING4)));
    println!("4. Result {:?}", syntax(&mut lexer::Lexer::new(STRING5)));
    println!("5. Result {:?}", syntax(&mut lexer::Lexer::new(STRING6)));
}
