use crate::complex::*;
#[derive(Debug, Clone)]
pub enum Operation<T: std::clone::Clone> {
    Add(Vec<Expr<T>>),
    Sub((Expr<T>, Expr<T>)), //a-b
    Mul(Vec<Expr<T>>),
    Div((Expr<T>, Expr<T>)), //a/b
    Pow((Expr<T>, Expr<T>)), //a^b
    Trig(TrigOp<T>),
    Log(Expr<T>),
    Exp(Expr<T>),
    Sqrt(Expr<T>),
    NthRoot((f64, Expr<T>)), //b^(1/a)
    Hyperbolic(HyperbolicOp<T>),
}
#[derive(Debug, Clone)]
pub enum TrigOp<T: Clone> {
    Sin(Expr<T>),
    Cos(Expr<T>),
    Tan(Expr<T>),
    Csc(Expr<T>),
    Sec(Expr<T>),
    Cot(Expr<T>),
}
#[derive(Debug, Clone)]
pub enum HyperbolicOp<T: Clone> {
    Sinh(Expr<T>),
    Cosh(Expr<T>),
    Tanh(Expr<T>),
    Csch(Expr<T>),
    Sech(Expr<T>),
    Coth(Expr<T>),
}
#[derive(Debug, Clone)]
pub enum Expr<T: Clone> {
    Variable(char),
    Constant(T),
    ComplexNum(Box<ComplexNumber<T>>),
    Operation(Box<Operation<T>>),
}

#[derive(Debug, Clone)]
pub enum ComplexNumber<T: Clone> {
    Cartesian(ComplexNumCartesianForm<T>),
    Polar(ComplexNumPolarForm<T>),
}
