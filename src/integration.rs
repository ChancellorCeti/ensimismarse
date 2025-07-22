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
    pub fn integrate(&self, variable: char) -> Expr<T> {
        let mut expanded_form = self.expand_product().1;
        expanded_form.simplify();
        let cleaned_problems = expanded_form.use_integration_linearity(variable);
        if cleaned_problems.len() > 1 {
            let mut solutions = vec![Expr::Constant(T::from(0.0)); cleaned_problems.len()];
            for i in 0..cleaned_problems.len() {
                solutions[i] = cleaned_problems[i].1.integrate(variable);
            }
            return Expr::Operation(Box::new(Operation::Add(
                (0..cleaned_problems.len())
                    .map(|i| {
                        Expr::Operation(Box::new(Operation::Mul(vec![
                            cleaned_problems[i].0.clone(),
                            solutions[i].clone(),
                        ])))
                    })
                    .collect(),
            )));
        }
        let mut selfclone = self.clone();
        selfclone.simplify();
        match selfclone {
            Expr::Constant(_c) => {
                return Expr::Operation(Box::new(Operation::Mul(vec![
                    self.clone(),
                    Expr::Variable(variable),
                ])))
            }
            Expr::Variable(v) => {
                if v.clone() != variable {
                    return Expr::Operation(Box::new(Operation::Mul(vec![
                        self.clone(),
                        Expr::Variable(variable),
                    ])));
                }
                return Expr::Operation(Box::new(Operation::Mul(vec![
                    Expr::Constant(T::from(0.5f64)),
                    Expr::Operation(Box::new(Operation::Pow((
                        Expr::Variable(variable),
                        Expr::Constant(T::from(2.0)),
                    )))),
                ])));
            }
            Expr::ComplexNum(_c) => {
                return Expr::Operation(Box::new(Operation::Mul(vec![
                    self.clone(),
                    Expr::Variable(variable),
                ])))
            }
            Expr::Operation(some_op) => match *some_op.to_owned() {
                Operation::Pow((base_expr, exponent_expr)) => {
                    if let Expr::Variable(base_var) = base_expr {
                        if let Expr::Constant(exponent_const) = exponent_expr
                            && base_var == variable
                        {
                            return Expr::Operation(Box::new(Operation::Mul(vec![
                                Expr::Operation(Box::new(Operation::Pow((
                                    Expr::Variable(variable),
                                    Expr::Constant(exponent_const.clone() + T::from(1.0)),
                                )))),
                                Expr::Constant(T::from(1.0) / (exponent_const + T::from(1.0))),
                            ])));
                        }
                    }
                }
                _ => {
                    return Expr::Constant(T::from(1.0f64));
                }
            },
        }
        return Expr::Constant(T::from(1.0f64));
    }
    fn find_sum_in_product(factors: &Vec<Expr<T>>) -> (bool, usize, usize) {
        let mut first_sum_index = 0;
        let mut sum_exists = false;
        let mut sum_count: usize = 0;
        for i in 0..factors.len() {
            if factors[i].check_if_sum() && sum_count == 0 {
                if sum_count == 0 {
                    sum_count += 1;
                    first_sum_index = i;
                    sum_exists = true;
                }
                if sum_count > 0 {
                    sum_count += 1;
                    break;
                }
            }
        }
        return (sum_exists, first_sum_index, sum_count);
    }
    //returns (bool,Self) where bool is true if there was a sum that needed expanding, true if not
    pub fn expand_product(&self) -> (bool, Self) {
        if let Expr::Operation(box Operation::Mul(factors)) = self {
            let (sum_exists, first_sum_index, sum_count) = Self::find_sum_in_product(factors);
            if sum_exists == false {
                let mut selfclone = self.clone();
                selfclone.simplify();
                return (false, selfclone);
            }
            let sum_a = factors[first_sum_index].clone();
            let mut other_factors = factors.clone();
            other_factors.remove(first_sum_index);
            // TO-DO: CHANGE CODE SO THE OTHER FACTOR IN NEW ELEMENTS OF RES_ADDENDS IS FACTORS BUT
            // WITHOUT THE SUM_A_ADDENDS
            let mut res_addends: Vec<Expr<T>> = vec![];
            if let Expr::Operation(box Operation::Add(sum_a_addends)) = sum_a {
                for i in 0..sum_a_addends.len() {
                    let mut other_factors_clone_i = other_factors.clone();
                    other_factors_clone_i.push(sum_a_addends[i].clone());
                    res_addends.push(Expr::Operation(Box::new(Operation::Mul(
                        other_factors_clone_i,
                    ))))
                }
            } else {
                panic!()
            };
            let mut res = (
                true,
                Expr::Operation(Box::new(Operation::Add(res_addends.clone()))),
            );
            res.1.simplify();
            if sum_count > 1 {
                let mut unsimplified_res = (
                    true,
                    Expr::Operation(Box::new(Operation::Add(
                        res_addends
                            .iter()
                            .map(|addend| addend.clone().expand_product().1)
                            .collect(),
                    ))),
                );
                unsimplified_res.1.simplify();
                unsimplified_res
            } else {
                return res;
            }
        } else {
            panic!("Expected product, found {}", self.expr_to_string());
        }
    }
    fn use_integration_linearity(&self, variable: char) -> Vec<(Expr<T>, Expr<T>)> {
        match self {
            Expr::Variable(_x) => return vec![(Expr::Constant(T::from(1.0)), self.clone())],
            Expr::Constant(_x) => return vec![(Expr::Constant(T::from(1.0)), self.clone())],
            Expr::ComplexNum(_x) => return vec![(Expr::Constant(T::from(1.0)), self.clone())],
            Expr::Operation(op) => match *op.to_owned() {
                Operation::Add(x) => {
                    return x
                        .clone()
                        .iter()
                        .map(|xi| xi.clone().use_integration_linearity(variable))
                        .flatten()
                        .collect();
                }
                Operation::Sub(x) => {
                    return vec![
                        (Expr::Constant(T::from(1.0)), x.0),
                        (
                            Expr::Constant(T::from(1.0)),
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
                    let mut constants_variables: Vec<Expr<T>> = vec![];
                    for i in 0..x.len() {
                        if let Expr::Constant(c) = &x[i] {
                            constants_exist = true;
                            constants_product = constants_product * c.clone();
                        }
                        if let Expr::Variable(c) = &x[i]
                            && *c != variable
                        {
                            constants_exist = true;
                            constants_variables.push(x[i].clone());
                        }
                        if let Some((variable_i, _power_i)) = x[i].check_if_constant_power_of_x() {
                            if variable_i != variable {
                                constants_variables.push(x[i].clone());
                            }
                        }
                    }

                    constants_variables.push(Expr::Constant(constants_product));
                    let mut res = x.clone();
                    let mut constants_product_expr =
                        Expr::Operation(Box::new(Operation::Mul(constants_variables)));
                    constants_product_expr.simplify();
                    if constants_exist {
                        res.retain(|factor| factor.check_if_constant() == false);
                        res.retain(|expr| {
                            if let Expr::Operation(box Operation::Pow((base, _exponent))) = expr {
                                if let Expr::Variable(var_name) = base {
                                    return var_name == &variable;
                                }
                            }
                            // Remove anything that doesn't match the pattern or has a different var name
                            false
                        });

                        /*for i in 0..res.len(){
                            if let Expr::Operation(box Operation::Pow((base,_exponent)))=res[i]{
                                if let Expr::Variable(var_name) = base{
                                    if var_name!=variable{

                                    }
                                }
                            }
                        }*/
                    }

                    return vec![(
                        constants_product_expr,
                        Expr::Operation(Box::new(Operation::Mul(res))),
                    )];
                }
                _ => {
                    return vec![(Expr::Constant(T::from(1.0)), self.clone())];
                }
            },
        }
    }
}
