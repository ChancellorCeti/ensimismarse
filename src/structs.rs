use crate::complex::*;
#[derive(Debug, Clone)]
pub enum Operation<T: std::clone::Clone + TrigOps> {
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
pub enum TrigOp<T: Clone + TrigOps> {
    Sin(Expr<T>),
    Cos(Expr<T>),
    Tan(Expr<T>),
    Csc(Expr<T>),
    Sec(Expr<T>),
    Cot(Expr<T>),
}
#[derive(Debug, Clone)]
pub enum HyperbolicOp<T: Clone + TrigOps> {
    Sinh(Expr<T>),
    Cosh(Expr<T>),
    Tanh(Expr<T>),
    Csch(Expr<T>),
    Sech(Expr<T>),
    Coth(Expr<T>),
}
#[derive(Debug, Clone)]
pub enum Expr<T: Clone + TrigOps> {
    Variable(char),
    Constant(T),
    ComplexNum(Box<ComplexNumber<T>>),
    Operation(Box<Operation<T>>),
}

#[derive(Debug, Clone)]
pub enum ComplexNumber<T: Clone + TrigOps> {
    Cartesian(ComplexNumCartesianForm<T>),
    Polar(ComplexNumPolarForm<T>),
}
pub trait TrigOps {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}
impl TrigOps for f64 {
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
}
