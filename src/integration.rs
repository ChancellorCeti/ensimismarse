use crate::structs::{Expr, Operation};

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
    pub fn integrate(&self, _variable: char) -> Expr<T> {
        println!(
            "ee {:#?}",
            self.use_integration_linearity()[0].0 /*.expr_to_string()*/
        );
        return Expr::Constant(T::from(0.0f64));
        /*match self{
        }*/
    }
    pub fn expand_product(&self) -> Self {
        if let Expr::Operation(box Operation::Mul(factors)) = self {
            /*let mut non_sum_factors: Vec<Expr<T>> = factors.clone();
            non_sum_factors.retain(|factor| factor.check_if_sum() == false);*/
            let mut sum_factors: Vec<Expr<T>> = factors.clone();
            sum_factors.retain(|factor| factor.check_if_sum() == true);
            // TO-DO: CHANGE CODE SO THE OTHER FACTOR IN NEW ELEMENTS OF RES_ADDENDS IS FACTORS BUT
            // WITHOUT THE SUM_A_ADDENDS
            let mut res_addends: Vec<Expr<T>> = vec![];
            if let Expr::Operation(box Operation::Add(sum_a_addends)) = &sum_factors[0] {
                for i in 0..sum_a_addends.len() {
                    res_addends.push(Expr::Operation(Box::new(Operation::Mul(vec![
                        sum_a_addends[i].clone(),
                    ]))))
                }
            } else {
                panic!()
            };
            for _i in 0..sum_factors.len() {
                /*res_addends.push(
                    Expr::Operation(Box::new(Operation::Mul(vec![
                        factors[i].clone()
                    ])))
                )*/
            }
            return Expr::Operation(Box::new(Operation::Mul(res_addends)));
        } else {
            panic!("Expected product, found {}", self.expr_to_string());
        }
    }
    fn use_integration_linearity(&self) -> Vec<(T, Expr<T>)> {
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

                    return vec![(
                        constants_product,
                        Expr::Operation(Box::new(Operation::Mul(res))),
                    )];
                }
                _ => {
                    return vec![(T::from(1.0), self.clone())];
                }
            },
        }
    }
}
