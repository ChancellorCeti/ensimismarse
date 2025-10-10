use crate::structs::{Expr, Operation, TrigOp, TrigOps};
use std::collections::HashMap;
pub fn differentiate<T>(f: Expr<T>, v: char) -> Expr<T>
where
    T: From<f64> + std::clone::Clone + TrigOps,
{
    let mut res: Expr<T>;
    res = Expr::Constant(T::from(0.0));
    let fc = f.clone();
    match f {
        Expr::ComplexNum(_z) => {
            todo!()
        }
        Expr::Constant(_x) => {
            res = Expr::Constant(T::from(0.0));
        }
        Expr::Variable(x) => {
            if x == v {
                res = Expr::Constant(T::from(1.0))
            } else {
                res = Expr::Constant(T::from(0.0))
            };
        }
        Expr::Operation(x) => match *x {
            Operation::Log(x) => {
                res = Expr::Operation(Box::new(Operation::Div((differentiate(x.clone(), v), x))))
            }
            Operation::Exp(x) => {
                res = Expr::Operation(Box::new(Operation::Mul(vec![
                    differentiate(x.clone(), v),
                    fc,
                ])))
            }
            Operation::Pow(x) => {
                res = Expr::Operation(Box::new(Operation::Mul(vec![
                    x.1.clone(),
                    differentiate(x.0.clone(), v),
                    Expr::Operation(Box::new(Operation::Pow((
                        x.0,
                        Expr::Operation(Box::new(Operation::Add(vec![
                            x.1,
                            Expr::Constant(T::from(-1.0)),
                        ]))),
                    )))),
                ])))
            }
            Operation::Trig(TrigOp::Sin(x)) => {
                res = Expr::Operation(Box::new(Operation::Mul(vec![
                    differentiate(x.clone(), v),
                    Expr::Operation(Box::new(Operation::Trig(TrigOp::Cos(x.clone())))),
                ])));
            }
            Operation::Trig(TrigOp::Cos(x)) => {
                res = Expr::Operation(Box::new(Operation::Mul(vec![
                    Expr::Constant(T::from(-1.0)),
                    differentiate(x.clone(), v),
                    Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(x.clone())))),
                ])));
            }
            Operation::Trig(TrigOp::Sec(x)) => {
                res = Expr::Operation(Box::new(Operation::Mul(vec![
                    differentiate(x.clone(), v),
                    Expr::Operation(Box::new(Operation::Trig(TrigOp::Sec(x.clone())))),
                    Expr::Operation(Box::new(Operation::Trig(TrigOp::Tan(x.clone())))),
                ])));
            }
            Operation::Trig(TrigOp::Csc(x)) => {
                res = Expr::Operation(Box::new(Operation::Mul(vec![
                    Expr::Constant(T::from(-1.0)),
                    differentiate(x.clone(), v),
                    Expr::Operation(Box::new(Operation::Trig(TrigOp::Csc(x.clone())))),
                    Expr::Operation(Box::new(Operation::Trig(TrigOp::Cot(x.clone())))),
                ])));
            }
            Operation::Mul(x) => {
                let mut xc = x.clone();
                match xc.len() {
                    1 => {
                        res = differentiate(xc[0].clone(), v);
                    }
                    0 => {
                        eprintln!("check this line, this really shouldnt happen bahaha");
                    }
                    _ => {
                        let x_last_factor = xc.pop().unwrap();
                        let mut res_addend_a = xc.clone();
                        res_addend_a.push(differentiate(x_last_factor.clone(), v));
                        let res_addend_a = Expr::Operation(Box::new(Operation::Mul(res_addend_a)));
                        let res_addend_b = Expr::Operation(Box::new(Operation::Mul(vec![
                            x_last_factor,
                            differentiate(Expr::Operation(Box::new(Operation::Mul(xc.clone()))), v),
                        ])));
                        res = Expr::Operation(Box::new(Operation::Add(vec![
                            res_addend_a,
                            res_addend_b,
                        ])));
                    }
                };
            }
            Operation::Add(x) => {
                let mut res_addends = Vec::new();
                for i in 0..x.len() {
                    res_addends.push(differentiate(x[i].clone(), v));
                }
                res = Expr::Operation(Box::new(Operation::Add(res_addends)));
            }
            Operation::Div((f, g)) => {
                let res_a = Expr::Operation(Box::new(Operation::Div((
                    differentiate(f.clone(), v),
                    g.clone(),
                ))));
                let res_b = Expr::Operation(Box::new(Operation::Div((
                    Expr::Operation(Box::new(Operation::Mul(vec![
                        Expr::Constant(T::from(-1.0)),
                        differentiate(g.clone(), v),
                        f.clone(),
                    ]))),
                    Expr::Operation(Box::new(Operation::Pow((
                        g.clone(),
                        Expr::Constant(T::from(2.0)),
                    )))),
                ))));
                res = Expr::Operation(Box::new(Operation::Add(vec![res_a, res_b])));
            }
            _ => todo!(),
        },
    }
    res
}
pub fn numerical_differentiate<T>(
    f: Expr<T>,
    v: char,
    acc: f64,
    variable_values: &HashMap<char, T>,
) -> T
where
    T: From<f64>
        + std::clone::Clone
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>
        + std::cmp::PartialEq
        + std::fmt::Debug
        + From<f64>
        + TrigOps
        + Into<f64>,
    f64: From<T>,
{
    let f1 = f.evaluate_expr(variable_values);
    let o = variable_values.get(&v);
    match o {
        Some(o_val) => {
            let o_val_c = o_val.clone();
            let mut variable_values_delta = variable_values.clone();
            variable_values_delta.insert(v, o_val_c + T::from(acc));
            let f2 = f.evaluate_expr(&variable_values_delta);
            let dy = f2 - f1;
            let dx = acc;
            return dy / dx.into();
        }
        None => {
            panic!("variable {} not defined with a numerical value when tryna numerically differentiate",v)
        }
    }
}
