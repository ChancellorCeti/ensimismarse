use crate::structs::{Expr,Operation,TrigOp};
pub fn differentiate<T>(f: Expr<T>, v: char) -> Expr<T>
where
    T: From<f64> + std::clone::Clone,
{
    let mut res: Expr<T>;
    res = Expr::Constant(T::from(0.0));
    let fc = f.clone();
    match f {
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
