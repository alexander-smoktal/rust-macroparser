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
    StatList(Vec<Box<Statement>>),
    Negate(Box<Statement>)
}

macro_rules! or {
    [$($parse_funcs: expr),+] => {
        |lexer: &mut lexer::Lexer| -> Option<Box<Statement>> {
            $(
                let result = $parse_funcs(lexer);

                if result.is_some() {
                    lexer.accept();
                    println!("Or accepted {:?}\n\twith lexer {:?}\n\tand func {:?}", result, lexer, stringify!($parse_funcs));
                    return result
                } else {
                    lexer.reject();
                    println!("Or didn't accept {:?}\n\twith lexer {:?}\n\tand func {:?}", result, lexer, stringify!($parse_funcs));
                }
            )+;

            println!("Or failed {:?}", result);
            None
        }
    }
}

macro_rules! and {
    [($($parse_funcs: expr),+) => $nandler_func: expr] => {
        |lexer: &mut lexer::Lexer| -> Option<Box<Statement>> {
            let results = ($(match $parse_funcs(lexer) {
                Some(statement) => {
                    println!("And accepted {:?}\n\twith lexer {:?}\n\tand func {:?}", statement, lexer, stringify!($parse_funcs));
                    statement
                }
                _ => {
                    println!("And didn't accept rule\n\twith lexer {:?}\n\tand func {:?}", lexer, stringify!($parse_funcs));
                    lexer.reject();
                    return None
                }
            }), +);

            match std::ops::Fn::call(&$nandler_func, results) {
                statement @ Some(_) => {
                    println!("And handling function successfully handled statement and returned {:?}", statement);
                    lexer.accept();
                    statement
                }
                _ => {
                    println!("And handling function failed to process statements");
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
            println!("Executing rule {:?}", stringify!($name));

            $parse_func(lexer)
        }
    };
}

fn num(lexer: &mut lexer::Lexer) -> Option<Box<Statement>> {
    println!("num {:?}", lexer);
    let result = lexer
        .next()
        .map(|c| c as char)
        .and_then(|c| {
                println!("----- NUMERIC {}", c);
                  if c.is_numeric() {
                      lexer.accept();
                      Some(Box::new(Statement::Number(c.to_string().parse::<f32>().unwrap())))
                  } else {
                      lexer.reject();
                      None
                  }});

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

fn make_operator(left: Box<Statement>, op: Box<Statement>, right: Box<Statement>) -> Option<Box<Statement>> {
    Some(Box::new(Statement::Operator{
        op,
        left,
        right
    }))
}

fn negate(_neg_token: Box<Statement>, number: Box<Statement>) -> Option<Box<Statement>> {
    Some(Box::new(Statement::Negate(number)))
}

// expr = sum
// sum = mul "+" sum | mul "-" sum | mul
// mul = atom "*" mul | atom "/" mul | atom
// atom = "(", expr , ")" | number | neg;
// neg = "-" atom

rule!(expr, sum);

rule!(sum, or![
     and![(mul, token('+'), sum) => make_operator],
     and![(mul, token('-'), sum) => make_operator],
     mul
]);

rule!(mul, or![
     and![(atom, token('*'), mul) => make_operator],
     and![(atom, token('/'), mul) => make_operator],
     atom
]);

rule!(atom, or![
    and![(token('('), expr, token(')')) => |_lbrace, stat, _rbrace| Some(stat)],
    num,
    neg
]);

rule!(neg, and![(token('-'), atom) => negate]);

const STRING1: &str = "1 + 2";
const STRING2: &str = "(1 + 2)";
const STRING3: &str = "(1 + 2) * 3";
const STRING4: &str = "1 + (2 + 3)";
const STRING5: &str = "(1 * 2) + (3 * 4)";
const STRING6: &str = "((1 * 2) + (3 + 4))";

fn main() {
    // let ref mut lex = lexer::Lexer::new("( ");
    // //rule!(test, and!(token('+'), token('(')));
    // println!("{:?}", test(lex));

    println!("0. Result {:?}", expr(&mut lexer::Lexer::new(STRING1)));
    // println!("1. Result {:?}", syntax(&mut lexer::Lexer::new(STRING2)));
    // println!("2. Result {:?}", syntax(&mut lexer::Lexer::new(STRING3)));
    // println!("3. Result {:?}", syntax(&mut lexer::Lexer::new(STRING4)));
    // println!("4. Result {:?}", syntax(&mut lexer::Lexer::new(STRING5)));
    // println!("5. Result {:?}", syntax(&mut lexer::Lexer::new(STRING6)));
}
