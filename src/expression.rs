#[derive(Debug)]
pub enum Expression {
    Operator {
        op: Box<Expression>,
        left: Box<Expression>,
        right: Box<Expression>
    },
    Number(f32),
    Token(char),
    Negate(Box<Expression>)
}

impl Expression {
    pub fn eval(self) -> f32 {
        match self {
            Expression::Operator { op: box Expression::Token('+'), left, right } => left.eval() + right.eval(),
            Expression::Operator { op: box Expression::Token('-'), left, right } => left.eval() - right.eval(),
            Expression::Operator { op: box Expression::Token('/'), left, right } => left.eval() / right.eval(),
            Expression::Operator { op: box Expression::Token('*'), left, right } => left.eval() * right.eval(),
            Expression::Number(val) => val,
            Expression::Negate(exp) => -exp.eval(),
            token => panic!("Got token inside an expression {:?}", token)
        }
    }
}
