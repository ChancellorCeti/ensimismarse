use crate::impls;
use crate::structs::{ComplexNumber, Expr, HyperbolicOp, Operation, TrigOp};
use std::collections::HashMap;

impl<
        T: std::clone::Clone
            + std::ops::Add<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Sub<Output = T>
            + std::cmp::PartialEq
            + std::fmt::Debug
            + From<f64>
            + Into<f64>,
    > Expr<T>
where
    f64: From<T>,
{
    pub fn integrate(&self, variable: char) -> Expr<T> {
        println!("{:#?}",self.use_integration_linearity()[0].0/*.expr_to_string()*/);
        return Expr::Constant(T::from(0.0f64));
        /*match self{
        }*/
    }
    fn use_integration_linearity(&self) -> Vec<(T, Expr<T>)>
    {
        match self {
            Expr::Variable(_x) => return vec![(T::from(1.0), self.clone())],
            Expr::Constant(_x) => return vec![(T::from(1.0), self.clone())],
            Expr::ComplexNum(_x) => return vec![(T::from(1.0), self.clone())],
            Expr::Operation(op) => match *op.to_owned() {
                Operation::Add(x) => {
                    return x
                        .clone()
                        .iter()
                        .map(|xi| xi.clone().use_integration_linearity())
                        .flatten()
                        .collect();
                }
                Operation::Sub(x) => {
                    return vec![
                        (T::from(1.0), x.0),
                        (
                            T::from(1.0),
                            Expr::Operation(Box::new(Operation::Mul(vec![
                                Expr::Constant(T::from(-1.0)),
                                x.1,
                            ]))),
                        ),
                    ]
                }
                Operation::Mul(x) => {
                    let mut constants_product: T = T::from(1.0);
                    let mut constants_exist = false;
                    for i in 0..x.len() {
                        if let Expr::Constant(c) = &x[i] {
                            constants_exist = true;
                            constants_product = constants_product * c.clone();
                        }
                    }
                    let mut res = x.clone();
                    if constants_exist {
                        res.retain(|factor| factor.check_if_constant() == false);
                    }

                    return vec![(constants_product, Expr::Operation(Box::new(Operation::Mul(res))))];
                }
                _ => {
                    return vec![(T::from(1.0), self.clone())];
                }
            },
        }
    }
}
fn expand_product<T>(f: &Expr<T>) -> Expr<T>
where
    T: std::clone::Clone,
{
    todo!()
}
