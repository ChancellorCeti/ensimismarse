#![allow(dead_code)]
use std::collections::HashMap;
use std::f64;
// TODO INVERSE TRIG AND HYPERBOLICS
mod differentiation;
mod impls;
mod structs;
use crate::differentiation::differentiate;
use crate::structs::{Expr, Operation, TrigOp};
use std::mem::discriminant;
fn main() {
    let a: Expr<f64> = Expr::Variable('x');
    let b: Expr<f64> = Expr::Variable('y');
    let xy_vals = HashMap::from([('x', 6.9), ('y', 0.3)]);
    let funky_boi = Expr::Operation(Box::new(Operation::Div((
        Expr::Operation(Box::new(Operation::Log(Expr::Operation(Box::new(
            Operation::Mul(vec![
                Expr::Operation(Box::new(Operation::Pow((a.clone(), Expr::Constant(3.0))))),
                Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(b.clone())))),
            ]),
        ))))),
        Expr::Operation(Box::new(Operation::Trig(TrigOp::Cos(Expr::Operation(
            Box::new(Operation::Pow((a.clone(), b.clone()))),
        ))))),
    ))));

    let mut funky_derivative = differentiate(funky_boi.clone(), 'x');
    funky_derivative.simplify();
    let c_val = funky_derivative.evaluate_expr(&xy_vals);
    println!("{}", c_val);
    //println!("{:#?}", funky_derivative);
    let s = Expr::Operation(Box::new(Operation::Mul(vec![
        Expr::Constant(0.1),
        Expr::Variable('x'),
    ])));
    let exy = Expr::Operation(Box::new(Operation::Mul(vec![
        Expr::Variable('x'),
        Expr::Constant(0.1),
    ])));
    println!("{}",s==exy);
    println!("{:?}",discriminant(&a));
    println!("{:?}",discriminant(&s));
    println!("{:?}",discriminant(&exy));
}
