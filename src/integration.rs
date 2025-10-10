use crate::complex::{ComplexNumCartesianForm, ComplexNumPolarForm};
use crate::structs::{ComplexNumber, Expr, Operation, TrigOp};

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
        //println!("expanded form is {:#?}", expanded_form.expr_to_string());
        //println!("expanded form is {:#?}", expanded_form);
        let cleaned_problems = expanded_form.use_integration_linearity(variable);
        if cleaned_problems.len() > 1 {
            let mut solutions = vec![Expr::Constant(T::from(0.0)); cleaned_problems.len()];
            for i in 0..cleaned_problems.len() {
                solutions[i] = cleaned_problems[i].1.integrate(variable);
            }
            let mut res = Expr::Operation(Box::new(Operation::Add(
                (0..cleaned_problems.len())
                    .map(|i| {
                        Expr::Operation(Box::new(Operation::Mul(vec![
                            cleaned_problems[i].0.clone(),
                            solutions[i].clone(),
                        ])))
                    })
                    .collect(),
            )));
            res.simplify();
            return res;
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
                Operation::Exp(argument) => {
                    let argument_checked_for_const_multiple =
                        argument.check_if_constant_multiple_of_x();
                    match argument_checked_for_const_multiple {
                        Some((x, coeff)) => {
                            if x == variable {
                                let res_exp = Expr::Operation(Box::new(Operation::Exp(
                                    Expr::Operation(Box::new(Operation::Mul(vec![
                                        coeff.clone(),
                                        Expr::Variable(x),
                                    ]))),
                                )));
                                let res_constant = match coeff {
                                    Expr::Constant(c) => Expr::Constant(T::from(1.0) / c),
                                    Expr::ComplexNum(c) => match *c.to_owned() {
                                        ComplexNumber::Polar(z) => {
                                            Expr::ComplexNum(Box::new(ComplexNumber::Polar(
                                                ComplexNumPolarForm {
                                                    modulus: T::from(1.0),
                                                    phase: T::from(0.0),
                                                } / z,
                                            )))
                                        }
                                        ComplexNumber::Cartesian(z) => {
                                            Expr::ComplexNum(Box::new(ComplexNumber::Polar(
                                                ComplexNumPolarForm {
                                                    modulus: T::from(1.0),
                                                    phase: T::from(0.0),
                                                } / z.to_polar(),
                                            )))
                                        }
                                    },
                                    Expr::Variable(_) | Expr::Operation(_) => todo!(),
                                };
                                return Expr::Operation(Box::new(Operation::Mul(vec![
                                    res_constant,
                                    res_exp,
                                ])));
                            }
                        }
                        None => {}
                    }
                }
                Operation::Mul(factors) => {
                    let cos_and_sin_check = Expr::check_if_nice_trig_product(&factors, variable);
                    if let Some(cos_and_sin_list) = cos_and_sin_check {
                        let mut factors_as_exps = factors.clone();
                        for i in 0..factors.len() {
                            factors_as_exps[i] = match &cos_and_sin_list[i] {
                                (0, xvar, coeff) => Expr::create_mul(vec![
                                    coeff.clone(),
                                    Expr::Variable(xvar.clone()),
                                ])
                                .create_complex_cosine_expr(),
                                (1, xvar, coeff) => Expr::create_mul(vec![
                                    coeff.clone(),
                                    Expr::Variable(xvar.clone()),
                                ])
                                .create_complex_sine_expr(),
                                _ => panic!(),
                            };
                        }
                        let mut new_integrand =
                            Expr::create_mul(factors_as_exps).expand_product().1;
                        new_integrand.simplify();
                        return new_integrand.integrate(variable);
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
    // checks if self is of form "5x" or "(2i+3)x" and if so returns Some(x,5) or Some(x,2i+3) etc
    fn check_if_constant_multiple_of_x(&self) -> Option<(char, Self)> {
        if let Expr::Operation(box Operation::Mul(factors)) = self {
            if factors.len() == 2 {
                if let Expr::Variable(x) = &factors[0] {
                    match &factors[1] {
                        Expr::Constant(_c) => {
                            return Some((*x, factors[1].clone()));
                        }
                        Expr::ComplexNum(_c) => {
                            return Some((*x, factors[1].clone()));
                        }
                        _ => {}
                    };
                }
                if let Expr::Variable(x) = &factors[1] {
                    match &factors[0] {
                        Expr::Constant(_c) => {
                            return Some((*x, factors[0].clone()));
                        }
                        Expr::ComplexNum(_c) => {
                            return Some((*x, factors[0].clone()));
                        }
                        _ => {}
                    };
                }
                return None;
            }
        }
        None
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
            return (false, self.clone());
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
                    // to-do implement \int (a+bi) f(x) = (a+bi) \int f(x)
                    let mut constants_product: T = T::from(1.0);
                    let mut constants_exist = false;
                    let mut complex_constants_product: ComplexNumCartesianForm<T> =
                        ComplexNumCartesianForm::create_cartesian_complex_num_simple(
                            T::from(1.0),
                            T::from(0.0),
                        );
                    let mut complex_constants_exist = false;

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
                        if let Expr::ComplexNum(c_cont) = &x[i] {
                            complex_constants_exist = true;
                            match *c_cont.to_owned() {
                                ComplexNumber::Cartesian(c) => {
                                    complex_constants_product =
                                        complex_constants_product * c.clone();
                                }
                                ComplexNumber::Polar(c) => {
                                    complex_constants_product =
                                        complex_constants_product * c.to_cartesian();
                                }
                            }
                        }
                        if let Some((variable_i, _power_i)) = x[i].check_if_constant_power_of_x() {
                            if variable_i != variable {
                                constants_variables.push(x[i].clone());
                            }
                        }
                    }

                    constants_variables.push(Expr::Constant(constants_product));
                    constants_variables.push(Expr::ComplexNum(Box::new(ComplexNumber::Cartesian(
                        complex_constants_product,
                    ))));
                    let mut res = x.clone();
                    let mut constants_product_expr =
                        Expr::Operation(Box::new(Operation::Mul(constants_variables)));
                    constants_product_expr.simplify();
                    if constants_exist {
                        res.retain(|factor| factor.check_if_constant() == false);
                        res.retain(|factor| factor.check_if_complex_constant() == false);
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
                    if complex_constants_exist {
                        res.retain(|factor| {
                            if let Expr::ComplexNum(_c_cont) = factor {
                                return false;
                            }
                            return true;
                        });
                    }
                    /*println!(
                        "const prod expr is {:?} and integrand is {:#?}",
                        constants_product_expr, res
                    );*/

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
    //Vec<usize> -- 0 if cosine, 1 if sine
    fn check_if_nice_trig_product(
        x: &Vec<Self>,
        variable: char,
    ) -> Option<Vec<(usize, char, Expr<T>)>> {
        let mut res = vec![(0, ' ', Expr::Variable('x')); x.len()];
        for i in 0..x.len() {
            match &x[i] {
                Expr::Operation(some_op) => match *some_op.to_owned() {
                    Operation::Trig(TrigOp::Sin(x)) => {
                        let is_x_nice = x.check_if_constant_multiple_of_x();
                        match is_x_nice {
                            Some((x_var, coeff)) => {
                                if x_var == variable {
                                    res[i] = (1, x_var, coeff);
                                } else {
                                    return None;
                                }
                            }
                            None => return None,
                        }
                    }
                    Operation::Trig(TrigOp::Cos(x)) => {
                        let is_x_nice = x.check_if_constant_multiple_of_x();
                        match is_x_nice {
                            Some((x_var, coeff)) => {
                                if x_var == variable {
                                    res[i] = (0, x_var, coeff);
                                } else {
                                    return None;
                                }
                            }
                            None => return None,
                        }
                    }
                    _ => {
                        return None;
                    }
                },
                _ => {
                    return None;
                }
            };
        }
        return Some(res);
    }
}
