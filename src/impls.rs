use crate::complex::ComplexNumCartesianForm;
use crate::complex::*;
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
    fn create_sum(addends: Vec<Self>) -> Self {
        Expr::Operation(Box::new(Operation::Add(addends)))
    }
    fn create_mul(factors: Vec<Self>) -> Self {
        Expr::Operation(Box::new(Operation::Mul(factors)))
    }
    fn create_exp(argument: Self) -> Self {
        Expr::Operation(Box::new(Operation::Exp(argument)))
    }
    pub fn create_complex_cosine_expr(&self) -> Self {
        Expr::Operation(Box::new(Operation::Div((
            Self::create_sum(vec![
                Self::create_exp(Self::create_mul(vec![
                    ComplexNumCartesianForm::create_cartesian_complex_num(
                        T::from(0.0),
                        T::from(1.0),
                    ),
                    self.clone(),
                ])),
                Self::create_exp(Self::create_mul(vec![
                    ComplexNumCartesianForm::create_cartesian_complex_num(
                        T::from(0.0),
                        T::from(-1.0),
                    ),
                    self.clone(),
                ])),
            ]),
            Expr::Constant(T::from(2.0)),
        ))))
    }

    pub fn create_complex_sine_expr(&self) -> Self {
        Expr::Operation(Box::new(Operation::Div((
            Self::create_sum(vec![
                Self::create_exp(Self::create_mul(vec![
                    ComplexNumCartesianForm::create_cartesian_complex_num(
                        T::from(0.0),
                        T::from(1.0),
                    ),
                    self.clone(),
                ])),
                Self::create_mul(vec![
                    Expr::Constant(T::from(-1.0)),
                    Self::create_exp(Self::create_mul(vec![
                        ComplexNumCartesianForm::create_cartesian_complex_num(
                            T::from(0.0),
                            T::from(-1.0),
                        ),
                        self.clone(),
                    ])),
                ]),
            ]),
            ComplexNumCartesianForm::create_cartesian_complex_num(T::from(0.0), T::from(2.0)),
        ))))
    }
    pub fn expr_to_string(&self) -> String {
        match self {
            Expr::ComplexNum(_z) => {
                todo!()
            }
            Expr::Constant(x) => {
                return format!("{:.3}", f64::from(x.clone()));
            }
            Expr::Variable(x) => {
                return x.to_string();
            }
            Expr::Operation(some_op) => match *some_op.to_owned() {
                Operation::Add(x) => {
                    let mut res: String = String::new();
                    for i in 0..x.len() - 1 {
                        if x[i].check_if_constant() || x[i].check_if_variable() {
                            res.push_str(x[i].expr_to_string().as_str());
                        } else {
                            res.push('(');
                            res.push_str(x[i].expr_to_string().as_str());
                            res.push(')');
                        }
                        res.push('+');
                    }
                    if x[x.len() - 1].check_if_constant() || x[x.len() - 1].check_if_variable() {
                        res.push_str(x[x.len() - 1].expr_to_string().as_str());
                    } else {
                        res.push('(');
                        res.push_str(x[x.len() - 1].expr_to_string().as_str());
                        res.push(')');
                    }
                    return res;
                }
                Operation::Mul(x) => {
                    let mut res: String = String::new();
                    if x.len() == 0 {
                        panic!("found a product with zero terms in it... strange");
                    }
                    for i in 0..x.len() - 1 {
                        if x[i].check_if_constant() || x[i].check_if_variable() {
                            res.push_str(x[i].expr_to_string().as_str());
                        } else {
                            res.push('(');
                            res.push_str(x[i].expr_to_string().as_str());
                            res.push(')');
                        }

                        res.push('*');
                    }
                    if x[x.len() - 1].check_if_constant() || x[x.len() - 1].check_if_variable() {
                        res.push_str(x[x.len() - 1].expr_to_string().as_str());
                    } else {
                        res.push('(');
                        res.push_str(x[x.len() - 1].expr_to_string().as_str());
                        res.push(')');
                    }
                    return res;
                }
                Operation::Div((a, b)) => {
                    let mut res: String = String::new();
                    if a.check_if_constant() || a.check_if_variable() {
                        res.push_str(a.expr_to_string().as_str());
                    } else {
                        res.push('(');
                        res.push_str(a.expr_to_string().as_str());
                        res.push(')');
                    }
                    res.push('/');
                    if b.check_if_constant() || b.check_if_variable() {
                        res.push_str(b.expr_to_string().as_str());
                    } else {
                        res.push('(');
                        res.push_str(b.expr_to_string().as_str());
                        res.push(')');
                    }
                    return res;
                }
                Operation::Pow((a, b)) => {
                    let mut res: String = String::new();
                    if a.check_if_constant() || a.check_if_variable() {
                        res.push_str(a.expr_to_string().as_str());
                    } else {
                        res.push('(');
                        res.push_str(a.expr_to_string().as_str());
                        res.push(')');
                    }
                    res.push('^');
                    if b.check_if_constant() || b.check_if_variable() {
                        res.push_str(b.expr_to_string().as_str());
                    } else {
                        res.push('(');
                        res.push_str(b.expr_to_string().as_str());
                        res.push(')');
                    }
                    return res;
                }
                Operation::Trig(TrigOp::Sin(x)) => {
                    let mut res: String = String::new();
                    res.push_str("sin(");
                    res.push_str(x.expr_to_string().as_str());
                    res.push(')');
                    return res;
                }
                Operation::Exp(x) => {
                    let mut res: String = String::new();
                    res.push_str("exp(");
                    res.push_str(x.expr_to_string().as_str());
                    res.push(')');
                    return res;
                }
                _ => {
                    todo!()
                }
            },
        }
    }
    fn check_if_zero(&self) -> bool {
        match self {
            Expr::Constant(x) => {
                if *x == Into::<T>::into(0.0) {
                    return true;
                } else {
                    return false;
                }
            }
            _ => false,
        }
    }
    pub fn check_if_constant(&self) -> bool {
        match self {
            Expr::Constant(_x) => {
                return true;
            }
            _ => false,
        }
    }

    pub fn check_if_constant_power_of_x(&self) -> Option<(char, T)> {
        if let Expr::Operation(x) = self {
            match *x.to_owned() {
                Operation::Pow((base, exponent)) => {
                    if base.check_if_variable() && exponent.check_if_constant() {
                        if let Expr::Variable(base_char) = base
                            && let Expr::Constant(exponent_val) = exponent
                        {
                            return Some((base_char, exponent_val));
                        } else {
                            panic!("check_if_variable and check_if_constant failed");
                        }
                    }
                    return None;
                }
                _ => {
                    return None;
                }
            }
        }
        return None;
    }
    pub fn check_if_sum(&self) -> bool {
        if let Expr::Operation(x) = self {
            match *x.to_owned() {
                Operation::Add(_x) => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        return false;
    }
    fn check_if_variable(&self) -> bool {
        if let Expr::Variable(_x) = self {
            return true;
        }
        return false;
    }
    fn _extract_operation(&self) -> Operation<T> {
        match self {
            Expr::Operation(x) => {
                let x_unboxed = *x.to_owned();
                return x_unboxed;
            }
            _ => {
                panic!("not an operation lol")
            }
        }
    }
    pub fn simplify(&mut self) {
        match self {
            Expr::ComplexNum(_z) => {
                return;
            }
            Expr::Variable(_x) => {
                return;
            }
            Expr::Constant(_x) => {
                return;
            }
            Expr::Operation(some_op) => match *some_op.to_owned() {
                Operation::Add(mut x) => {
                    let mut constants_sum: T = (0.0).into();
                    let mut constants_exist = false;
                    let mut constants_count: usize = 0;
                    for i in 0..x.len() {
                        x[i].simplify();
                        if let Expr::Constant(ref c) = x[i] {
                            constants_count += 1;
                            constants_exist = true;
                            constants_sum = constants_sum + c.clone();
                        }
                        //check for af(x)+bf(x)=(a+b)f(x) where a,b are real numbers
                        //todo!!!!!!
                        /*if let Expr::Operation(addend_op) = &x[i] {
                            let addend_op_unboxed = *addend_op.to_owned();
                            if let Operation::Mul(addend_factors) = addend_op_unboxed {
                                if addend_factors.len() == 2 {
                                    if let Expr::Constant(ref c) = addend_factors[1] {
                                        if addend_factors[0].check_if_operation() {}
                                        if addend_factors[0].check_if_variable() {}
                                    }
                                    if let Expr::Constant(ref c) = addend_factors[0] {
                                        if addend_factors[1].check_if_operation() {}
                                        if addend_factors[1].check_if_variable() {}
                                    }
                                    let addend_a = addend_factors[0].extract_operation();
                                    let addend_b = addend_factors[1].extract_operation();
                                }
                            }
                        }*/
                    }
                    if constants_count == x.len() {
                        *self = Expr::Constant(constants_sum);
                        return;
                    }
                    if constants_exist {
                        x.retain(|addend| addend.check_if_constant() == false);
                        if constants_sum != (0.0).into() {
                            x.push(Expr::Constant(constants_sum));
                        }
                    }
                    *self = Expr::Operation(Box::new(Operation::Add(x)))
                }
                Operation::Div((mut a, mut b)) => {
                    a.simplify();
                    b.simplify();
                    // todo -- implement (xy)/(xz)=y/z
                    // todo -- implement [f(x)sinx]/[g(x)cosx]=[f(x)/g(x)]tanx etc for trig
                    // functions
                    if a.check_if_zero() {
                        *self = Expr::Constant((0.0).into());
                        return;
                    }
                    *self = Expr::Operation(Box::new(Operation::Div((a, b))));
                }
                Operation::Mul(mut x) => {
                    if x.len() == 1 {
                        *self = x[0].clone();
                        return;
                    }
                    let mut res_factors: Vec<Expr<T>> = Vec::new();
                    let mut constants_exist = false;
                    let mut constants_count: usize = 0;
                    let mut constants_sum: T = (1.0).into();
                    let mut vars_count: HashMap<char, T> = HashMap::new();
                    //for i in dd
                    //check if any factor is equal to 0, set the whole thing to 0 if so
                    for i in 0..x.len() {
                        let pow_of_variable = x[i].check_if_constant_power_of_x();
                        //if pow_of_variable == None {
                        x[i].simplify();
                        //}
                        if x[i].check_if_zero() {
                            *self = Expr::Constant((0.0).into());
                            return;
                        }
                        if x[i].check_if_variable() {
                            let var_i = match x[i] {
                                Self::Variable(c) => c,
                                _ => panic!("expected variable, found something else"),
                            };
                            match vars_count.get_mut(&var_i) {
                                Some(var_i_count) => {
                                    *var_i_count = var_i_count.clone() + T::from(1.0);
                                }
                                None => {
                                    vars_count.insert(var_i, T::from(1.0));
                                }
                            };
                        }
                        if let Some((var_i, power_i)) = pow_of_variable {
                            match vars_count.get_mut(&var_i) {
                                Some(var_i_count) => {
                                    *var_i_count = power_i + var_i_count.clone();
                                }
                                None => {
                                    vars_count.insert(var_i, power_i);
                                }
                            };
                        }
                        if let Expr::Constant(ref c) = x[i] {
                            constants_count += 1;
                            constants_exist = true;
                            constants_sum = constants_sum * c.clone();
                        }
                        res_factors.push(x[i].clone());
                    }
                    if constants_count == x.len() {
                        *self = Expr::Constant(constants_sum);
                        return;
                    }
                    if constants_exist {
                        res_factors.retain(|addend| addend.check_if_constant() == false);
                        if constants_sum != T::from(1.0) {
                            res_factors.push(Expr::Constant(constants_sum));
                        }
                    }
                    res_factors.retain(|factor| None == factor.check_if_constant_power_of_x());
                    res_factors.retain(|factor| factor.check_if_variable() == false);
                    for var_letter in vars_count.keys() {
                        let var_i_power = vars_count.get(var_letter).unwrap().clone();
                        if var_i_power != T::from(1.0) {
                            res_factors.push(Expr::Operation(Box::new(Operation::Pow((
                                Expr::Variable(*var_letter),
                                Expr::Constant(vars_count.get(var_letter).unwrap().clone()),
                            )))));
                        } else {
                            res_factors.push(Expr::Variable(var_letter.clone()));
                        }
                    }
                    *self = Expr::Operation(Box::new(Operation::Mul(res_factors)));
                }
                Operation::Pow((mut a, mut b)) => {
                    a.simplify();
                    b.simplify();
                    if a.check_if_zero() {
                        *self = Expr::Constant((0.0).into());
                        return;
                    }
                    if let Expr::Constant(ref c) = &b {
                        if *c == (0.0).into() {
                            *self = Expr::Constant((1.0).into());
                            return;
                        }
                    }
                    *self = Expr::Operation(Box::new(Operation::Pow((a, b))))
                }
                _ => {}
            },
        }
    }
    pub fn evaluate_expr(&self, variable_values: &HashMap<char, T>) -> T {
        match self {
            Expr::ComplexNum(_z) => panic!("found a complex number when using evaluate_expr. please use evaluate_complex_expr if complex numbers might appear in a calculation"),
            Expr::Variable(x) => {
                let var_desired = variable_values.get(x);
                match var_desired {
                    Some(var_value) => return var_value.clone(),
                    None => {
                        panic!(
                            "didnt provide value for one of the variables lol, it was {}",
                            x
                        )
                    }
                }
            }
            Expr::Constant(x) => return x.clone(),
            Expr::Operation(x) => match *x.to_owned() {
                Operation::Add(x) => {
                    let mut res: T = (0.0).into();
                    for i in 0..x.len() {
                        res = x[i].evaluate_expr(variable_values) + res;
                    }
                    return res;
                }
                Operation::Mul(x) => {
                    let mut res: T = (1.0).into();
                    for i in 0..x.len() {
                        res = x[i].evaluate_expr(variable_values) * res;
                    }
                    return res;
                }
                Operation::Div((a, b)) => {
                    return a.evaluate_expr(variable_values) / b.evaluate_expr(variable_values);
                }
                Operation::Pow((a, b)) => {
                    return f64::from(a.evaluate_expr(variable_values))
                        .powf(f64::from(b.evaluate_expr(variable_values)))
                        .into();
                }
                Operation::Log(x) => {
                    return f64::from(x.evaluate_expr(variable_values)).ln().into();
                }
                Operation::Exp(x) => {
                    return f64::from(x.evaluate_expr(variable_values)).exp().into();
                }
                Operation::Sub((a, b)) => {
                    return a.evaluate_expr(variable_values) - b.evaluate_expr(variable_values);
                }
                Operation::Sqrt(x) => {
                    return f64::from(x.evaluate_expr(variable_values)).sqrt().into();
                }
                Operation::NthRoot((a, b)) => {
                    return f64::from(b.evaluate_expr(variable_values))
                        .powf(1.0 / a)
                        .into();
                }
                Operation::Trig(TrigOp::Sin(x)) => {
                    return f64::from(x.evaluate_expr(variable_values)).sin().into();
                }
                Operation::Trig(TrigOp::Cos(x)) => {
                    return f64::from(x.evaluate_expr(variable_values)).cos().into();
                }
                Operation::Trig(TrigOp::Tan(x)) => {
                    return f64::from(x.evaluate_expr(variable_values)).tan().into();
                }
                Operation::Trig(TrigOp::Sec(x)) => {
                    return (1.0 / f64::from(x.evaluate_expr(variable_values)).cos()).into();
                }
                Operation::Trig(TrigOp::Csc(x)) => {
                    return (1.0 / f64::from(x.evaluate_expr(variable_values)).sin()).into();
                }
                Operation::Trig(TrigOp::Cot(x)) => {
                    return (1.0 / f64::from(x.evaluate_expr(variable_values)).tan()).into();
                }
                Operation::Hyperbolic(HyperbolicOp::Sinh(x)) => {
                    return f64::from(x.evaluate_expr(variable_values)).sinh().into();
                }
                Operation::Hyperbolic(HyperbolicOp::Cosh(x)) => {
                    return f64::from(x.evaluate_expr(variable_values)).cosh().into();
                }
                Operation::Hyperbolic(HyperbolicOp::Tanh(x)) => {
                    return f64::from(x.evaluate_expr(variable_values)).tanh().into();
                }
                Operation::Hyperbolic(HyperbolicOp::Csch(x)) => {
                    return (1.0 / f64::from(x.evaluate_expr(variable_values)).sinh()).into();
                }
                Operation::Hyperbolic(HyperbolicOp::Sech(x)) => {
                    return (1.0 / f64::from(x.evaluate_expr(variable_values)).cosh()).into();
                }
                Operation::Hyperbolic(HyperbolicOp::Coth(x)) => {
                    return (1.0 / f64::from(x.evaluate_expr(variable_values)).tanh()).into();
                }
            },
        }
    }
    pub fn evaluate_complex_expr(
        &self,
        variable_values: &HashMap<char, ComplexNumber<T>>,
    ) -> ComplexNumber<T> {
        match self {
            Expr::ComplexNum(z) => return *z.to_owned(),
            Expr::Variable(x) => {
                let var_desired = variable_values.get(x);
                match var_desired {
                    Some(var_value) => return var_value.clone(),
                    None => {
                        panic!(
                            "didnt provide value for one of the variables lol, it was {}",
                            x
                        )
                    }
                }
            }
            Expr::Constant(x) => {
                return ComplexNumber::Cartesian(ComplexNumCartesianForm {
                    real_part: x.clone(),
                    imaginary_part: T::from(0.0),
                })
            }

            Expr::Operation(x) => match *x.to_owned() {
                Operation::Add(x) => {
                    let mut res: ComplexNumCartesianForm<T> = ComplexNumCartesianForm {
                        real_part: T::from(0.0),
                        imaginary_part: T::from(0.0),
                    };
                    for i in 0..x.len() {
                        let res_i = x[i].evaluate_complex_expr(variable_values);
                        match res_i {
                            ComplexNumber::Cartesian(z) => res = res + z,
                            ComplexNumber::Polar(z) => res = res + z.to_cartesian(),
                        }
                    }
                    return ComplexNumber::Cartesian(res);
                }
                Operation::Mul(x) => {
                    let mut res: ComplexNumPolarForm<T> = ComplexNumPolarForm {
                        modulus: T::from(1.0),
                        phase: T::from(0.0),
                    };
                    for i in 0..x.len() {
                        let res_i = x[i].evaluate_complex_expr(variable_values);
                        match res_i {
                            ComplexNumber::Cartesian(z) => res = res * z.to_polar(),
                            ComplexNumber::Polar(z) => res = res * z,
                        }
                    }
                    return ComplexNumber::Polar(res);
                }
                Operation::Div((x, y)) => {
                    let x_eval = x.evaluate_complex_expr(variable_values);
                    let y_eval = y.evaluate_complex_expr(variable_values);
                    let zx_polar = match x_eval {
                        ComplexNumber::Polar(ref z) => z.clone(),
                        ComplexNumber::Cartesian(ref z) => z.to_polar(),
                    };

                    let zy_polar = match y_eval {
                        ComplexNumber::Polar(ref z) => z.clone(),
                        ComplexNumber::Cartesian(ref z) => z.to_polar(),
                    };

                    return ComplexNumber::Polar(zx_polar / zy_polar);
                }
                Operation::Exp(x) => {
                    let x_eval = x.evaluate_complex_expr(variable_values);
                    let x_cartesian = match x_eval {
                        ComplexNumber::Polar(ref z) => z.to_cartesian(),
                        ComplexNumber::Cartesian(ref z) => z.clone(),
                    };
                    return ComplexNumber::Polar(ComplexNumPolarForm {
                        modulus: T::from(f64::from(x_cartesian.real_part).exp()),
                        phase: x_cartesian.imaginary_part,
                    });
                }
                Operation::Trig(TrigOp::Sin(x)) => x
                    .create_complex_sine_expr()
                    .evaluate_complex_expr(variable_values),
                Operation::Trig(TrigOp::Cos(x)) => x
                    .create_complex_cosine_expr()
                    .evaluate_complex_expr(variable_values),
                _ => ComplexNumber::Cartesian(ComplexNumCartesianForm {
                    real_part: T::from(0.0),
                    imaginary_part: T::from(0.0),
                }),
            },
        }
    }
}
impl<T: std::clone::Clone + std::cmp::PartialEq + std::cmp::PartialEq> PartialEq for Expr<T> {
    fn eq(&self, other: &Self) -> bool {
        use std::mem::discriminant;
        match (self, other) {
            (Expr::Variable(x), Expr::Variable(y)) => return x == y,
            (Expr::Constant(x), Expr::Constant(y)) => return x == y,
            (Expr::Operation(op_x_box), Expr::Operation(op_y_box)) => {
                let op_x = *op_x_box.to_owned();
                let op_y = *op_y_box.to_owned();
                let x_op_type = discriminant(&op_x);
                let y_op_type = discriminant(&op_y);
                if x_op_type != y_op_type {
                    return false;
                }
                match (op_x, op_y) {
                    (Operation::Add(x), Operation::Add(y)) => {
                        let count = x
                            .iter()
                            .filter(|item| y.iter().any(|other| *item == other))
                            .count();
                        return x.len() == count;
                    }
                    (Operation::Mul(x), Operation::Mul(y)) => {
                        let count = x
                            .iter()
                            .filter(|item| y.iter().any(|other| *item == other))
                            .count();
                        return x.len() == count;
                    }
                    (Operation::Pow((xa, xb)), Operation::Pow((ya, yb))) => {
                        return (xa == ya) && (xb == yb);
                    }
                    (Operation::Exp(x), Operation::Exp(y)) => {
                        return x == y;
                    }
                    (Operation::Log(x), Operation::Log(y)) => {
                        return x == y;
                    }
                    (Operation::Trig(TrigOp::Sin(x)), Operation::Trig(TrigOp::Sin(y))) => {
                        return x == y;
                    }
                    (Operation::Trig(TrigOp::Cos(x)), Operation::Trig(TrigOp::Cos(y))) => {
                        return x == y;
                    }
                    (Operation::Trig(TrigOp::Tan(x)), Operation::Trig(TrigOp::Tan(y))) => {
                        return x == y;
                    }
                    (Operation::Trig(TrigOp::Sec(x)), Operation::Trig(TrigOp::Sec(y))) => {
                        return x == y;
                    }
                    (Operation::Trig(TrigOp::Csc(x)), Operation::Trig(TrigOp::Csc(y))) => {
                        return x == y;
                    }
                    (Operation::Trig(TrigOp::Cot(x)), Operation::Trig(TrigOp::Cot(y))) => {
                        return x == y;
                    }
                    _ => {}
                }
                return false;
            }
            _ => {
                return false;
            }
        }
    }
}
