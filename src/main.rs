#![feature(conservative_impl_trait)]
#![feature(trace_macros)]
#![feature(fn_traits)]
#![feature(box_patterns)]

//trace_macros!(true);

mod lexer;
mod expression;

use std::boxed::Box;

const DEBUG: bool = false;

macro_rules! debug_parser {
    ($($params: expr), +) => {
        if DEBUG {
            println!($($params,) +);
        }
    };
}

macro_rules! or {
    [$($parse_funcs: expr),+] => {
        |lexer: &mut lexer::Lexer| -> Option<Box<expression::Expression>> {
            $(
                let parser_pos = lexer.position();
                let result = $parse_funcs(lexer);

                if result.is_some() {
                    debug_parser!("Or statement rule {} accepted expression {:?}. Lexer state {:?}", stringify!($parse_funcs), result, lexer);
                    return result
                } else {
                    lexer.rollback(parser_pos);
                    debug_parser!("Or statement rule {} didn't accept lexer input {:?}", stringify!($parse_funcs), lexer);
                }
            )+;

            debug_parser!("Or statement fails");
            None
        }
    }
}

macro_rules! and {
    [($($parse_funcs: expr),+) => $nandler_func: expr] => {
        |lexer: &mut lexer::Lexer| -> Option<Box<expression::Expression>> {
            let parser_pos = lexer.position();

            let results = ($(match $parse_funcs(lexer) {
                Some(expression) => {
                    debug_parser!("And statement rule {} accepted expression {:?}. Lexer state {:?}", stringify!($parse_funcs), expression, lexer);
                    expression
                }
                _ => {
                    debug_parser!("And statement rule {} didn't accept lexer input {:?}", stringify!($parse_funcs), lexer);
                    lexer.rollback(parser_pos);
                    return None
                }
            }), +);

            match std::ops::Fn::call(&$nandler_func, results) {
                expression @ Some(_) => {
                    debug_parser!("And handling function successfully handled expression and returned {:?}", expression);
                    expression
                }
                _ => {
                    debug_parser!("And handling function failed to process expressions");
                    lexer.rollback(parser_pos);
                    return None
                }
            }
        }
    };
}

macro_rules! rule {
    ($name: ident, $parse_func:expr) => {
        fn $name(lexer: &mut lexer::Lexer) -> Option<Box<expression::Expression>> {
            debug_parser!("Executing rule {}", stringify!($name));

            $parse_func(lexer)
        }
    };
}

fn num(lexer: &mut lexer::Lexer) -> Option<Box<expression::Expression>> {
    let parser_pos = lexer.position();

    let result = lexer
        .next()
        .map(|c| c as char)
        .and_then(|c| {
                  if c.is_numeric() {
                      Some(Box::new(expression::Expression::Number(c.to_string().parse::<f32>().unwrap())))
                  } else {
                      lexer.rollback(parser_pos);
                      None
                  }});

    result
}

fn token(token_char: char) -> impl FnMut(&mut lexer::Lexer) -> Option<Box<expression::Expression>> {
    move |ref mut lexer| {
        let parser_pos = lexer.position();

        lexer
        .next()
        .map(|c| c as char).and_then(|c| 
            if c == token_char {
                Some(Box::new(expression::Expression::Token(c)))
            } else {
                lexer.rollback(parser_pos);
                None
            })
    }
}

fn make_operator(left: Box<expression::Expression>, op: Box<expression::Expression>, right: Box<expression::Expression>) -> Option<Box<expression::Expression>> {
    Some(Box::new(expression::Expression::Operator{
        op,
        left,
        right
    }))
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

rule!(neg, and![(token('-'), atom) => |_, number| Some(Box::new(expression::Expression::Negate(number)))]);

fn main() {
    let result0 = expr(&mut lexer::Lexer::new("1 + 2"));
    let result1 = expr(&mut lexer::Lexer::new("(1 + -2)"));
    let result2 = expr(&mut lexer::Lexer::new("(1 + 2) * 3"));
    let result3 = expr(&mut lexer::Lexer::new("1 * (2 - 3)"));
    let result4 = expr(&mut lexer::Lexer::new("1 * -2 + 3 * 4"));
    let result5 = expr(&mut lexer::Lexer::new("(1 * 2 + (-3 + -4))"));

    println!("0. Result {:?}", result0);
    println!("1. Result {:?}", result1);
    println!("2. Result {:?}", result2);
    println!("3. Result {:?}", result3);
    println!("4. Result {:?}", result4);
    println!("5. Result {:?}", result5);

    assert_eq!(result0.unwrap().eval(), 1f32 + 2f32);
    assert_eq!(result1.unwrap().eval(), 1f32 - 2f32);
    assert_eq!(result2.unwrap().eval(), (1f32 + 2f32) * 3f32);
    assert_eq!(result3.unwrap().eval(), 1f32 * (2f32 - 3f32));
    assert_eq!(result4.unwrap().eval(), 1f32 * -2f32 + 3f32 * 4f32);
    assert_eq!(result5.unwrap().eval(), 1f32 * 2f32 + (-3f32 + -4f32));
}
