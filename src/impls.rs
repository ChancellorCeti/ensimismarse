use crate::complex::ComplexNumCartesianForm;
use crate::complex::*;
use crate::structs::{ComplexNumber, Expr, HyperbolicOp, Operation, TrigOp, TrigOps};
use std::collections::HashMap;

impl<
        T: std::clone::Clone
            + std::ops::Add<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Sub<Output = T>
            + std::cmp::PartialEq
            + std::fmt::Debug
            + TrigOps
            + From<f64>
            + Into<f64>,
    > Expr<T>
where
    f64: From<T>,
    T: TrigOps,
{
    fn check_if_is_sum(&self) -> bool {
        if let Expr::Operation(box some_op) = &self {
            if let Operation::Add(_nested_sum) = some_op {
                return true;
            }
        }
        return false;
    }
    fn check_if_exp(&self) -> bool {
        if let Expr::Operation(box some_op) = &self {
            if let Operation::Exp(_argument) = some_op {
                return true;
            }
        }
        return false;
    }
    pub fn simplify_complex_exps(&mut self) {
        match self {
            Expr::Operation(some_op) => match *some_op.to_owned() {
                Operation::Add(mut x) => {
                    turn_complex_exps_into_trigs(&mut x);
                    *self = Expr::Operation(Box::new(Operation::Add(x)));
                }
                _ => {}
            },
            _ => {}
        }
    }
    fn check_if_is_mul(&self) -> bool {
        if let Expr::Operation(box some_op) = &self {
            if let Operation::Mul(_nested_product) = some_op {
                return true;
            }
        }
        return false;
    }
    pub fn create_sum(addends: Vec<Self>) -> Self {
        Expr::Operation(Box::new(Operation::Add(addends)))
    }
    pub fn create_mul(factors: Vec<Self>) -> Self {
        Expr::Operation(Box::new(Operation::Mul(factors)))
    }
    pub fn create_exp(argument: Self) -> Self {
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
            Expr::ComplexNum(z_wrapped) => match &**z_wrapped {
                ComplexNumber::Cartesian(z) => {
                    return format!(
                        "{:.9}+{:.9}i",
                        f64::from(z.real_part.clone()),
                        f64::from(z.imaginary_part.clone())
                    );
                }
                ComplexNumber::Polar(z) => {
                    return Expr::ComplexNum(Box::new(ComplexNumber::Cartesian(z.to_cartesian())))
                        .expr_to_string();
                }
            },
            Expr::Constant(x) => {
                return format!("{:.8}", f64::from(x.clone()));
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
                Operation::Trig(TrigOp::Cos(x)) => {
                    let mut res: String = String::new();
                    res.push_str("cos(");
                    res.push_str(x.expr_to_string().as_str());
                    res.push(')');
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
                    panic!("{:?}", self);
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
    pub fn check_if_complex_constant(&self) -> bool {
        match self {
            Expr::Constant(_x) => {
                return true;
            }
            Expr::ComplexNum(_z) => {
                return true;
            }
            _ => false,
        }
    }

    pub fn extract_if_variable(&self) -> Option<char> {
        if let Expr::Variable(x) = self {
            return Some(*x);
        }
        return None;
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
        self.simplify_with_options(false)
    }
    pub fn simplify_with_options(&mut self, cheese_complex_nums_to_constants: bool) {
        match self {
            Expr::ComplexNum(z_cont) => {
                if cheese_complex_nums_to_constants {
                    match *z_cont.to_owned() {
                        ComplexNumber::Cartesian(z) => {
                            if f64::from(z.imaginary_part).abs() < (10.0f64).powi(-14) {
                                *self = Expr::Constant(z.real_part);
                                return;
                            }
                        }
                        ComplexNumber::Polar(z) => {
                            if f64::from(z.to_cartesian().imaginary_part).abs()
                                < (10.0f64).powi(-14)
                            {
                                *self = Expr::Constant(z.to_cartesian().real_part);
                                return;
                            }
                        }
                    }
                }
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
                    let mut complex_const_count: usize = 0;
                    let mut complex_const_exist = false;
                    let mut complex_const_sum: ComplexNumCartesianForm<T> =
                        ComplexNumCartesianForm::create_cartesian_complex_num_simple(
                            T::from(0.0),
                            T::from(0.0),
                        );
                    let mut nested_addends: Vec<Expr<T>> = vec![];
                    let mut var_pows_coeff: HashMap<(char, u64), ComplexNumCartesianForm<T>> =
                        HashMap::new();
                    if x.len() == 1 {
                        let mut x0clone = x[0].clone();
                        x0clone.simplify_with_options(cheese_complex_nums_to_constants);
                        *self = x0clone;
                        return;
                    }
                    for i in 0..x.len() {
                        x[i].simplify_with_options(cheese_complex_nums_to_constants);
                        if let Expr::Operation(box some_op) = &x[i] {
                            if let Operation::Add(nested_sum) = some_op {
                                for nested_addend in nested_sum {
                                    nested_addends.push(nested_addend.clone());
                                }
                            }
                            /* start of code that does ax^n+bx^n=(a+b)x^n*/
                            if let Operation::Mul(nested_prod) = some_op {
                                if nested_prod.len() == 2 {
                                    if (nested_prod[1].check_if_complex_constant()
                                        || nested_prod[1].check_if_constant())
                                        && nested_prod[0].check_if_variable()
                                    {
                                        let x_pow = (
                                            nested_prod[0].extract_if_variable().unwrap(),
                                            T::from(1.0),
                                        );

                                        match &nested_prod[1] {
                                            Expr::ComplexNum(box ComplexNumber::Cartesian(z)) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + z.clone();
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            z.clone(),
                                                        );
                                                    }
                                                };
                                            }
                                            Expr::ComplexNum(box ComplexNumber::Polar(z)) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + z.to_cartesian();
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            z.to_cartesian(),
                                                        );
                                                    }
                                                };
                                            }
                                            Expr::Constant(c) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + ComplexNumCartesianForm::create_cartesian_complex_num_simple(c.clone(),T::from(0.0));
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            ComplexNumCartesianForm::create_cartesian_complex_num_simple(c.clone(),T::from(0.0)),
                                                        );
                                                    }
                                                };
                                            }
                                            _ => {}
                                        }
                                    }

                                    if (nested_prod[0].check_if_complex_constant()
                                        || nested_prod[0].check_if_constant())
                                        && nested_prod[1].check_if_variable()
                                    {
                                        let x_pow = (
                                            nested_prod[1].extract_if_variable().unwrap(),
                                            T::from(1.0),
                                        );

                                        match &nested_prod[0] {
                                            Expr::ComplexNum(box ComplexNumber::Cartesian(z)) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + z.clone();
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            z.clone(),
                                                        );
                                                    }
                                                };
                                            }
                                            Expr::ComplexNum(box ComplexNumber::Polar(z)) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        //println!("found var {:?} with current coeff {:?}", x_pow,coeff);
                                                        *coeff = coeff.clone() + z.to_cartesian();
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            z.to_cartesian(),
                                                        );
                                                    }
                                                };
                                            }
                                            Expr::Constant(c) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + ComplexNumCartesianForm::create_cartesian_complex_num_simple(c.clone(),T::from(0.0));
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            ComplexNumCartesianForm::create_cartesian_complex_num_simple(c.clone(),T::from(0.0)),
                                                        );
                                                    }
                                                };
                                            }
                                            _ => {}
                                        }
                                    }

                                    if (nested_prod[0].check_if_complex_constant()
                                        || nested_prod[0].check_if_constant())
                                        && nested_prod[1].check_if_constant_power_of_x().is_some()
                                    {
                                        let x_pow =
                                            nested_prod[1].check_if_constant_power_of_x().unwrap();

                                        match &nested_prod[0] {
                                            Expr::ComplexNum(box ComplexNumber::Cartesian(z)) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + z.clone();
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            z.clone(),
                                                        );
                                                    }
                                                };
                                            }
                                            Expr::ComplexNum(box ComplexNumber::Polar(z)) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + z.to_cartesian();
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            z.to_cartesian(),
                                                        );
                                                    }
                                                };
                                            }
                                            Expr::Constant(c) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + ComplexNumCartesianForm::create_cartesian_complex_num_simple(c.clone(),T::from(0.0));
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            ComplexNumCartesianForm::create_cartesian_complex_num_simple(c.clone(),T::from(0.0)),
                                                        );
                                                    }
                                                };
                                            }
                                            _ => {}
                                        }
                                    }
                                    if (nested_prod[1].check_if_complex_constant()
                                        || nested_prod[1].check_if_constant())
                                        && nested_prod[0].check_if_constant_power_of_x().is_some()
                                    {
                                        let x_pow =
                                            nested_prod[0].check_if_constant_power_of_x().unwrap();

                                        match &nested_prod[1] {
                                            Expr::ComplexNum(box ComplexNumber::Cartesian(z)) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + z.clone();
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            z.clone(),
                                                        );
                                                    }
                                                };
                                            }
                                            Expr::ComplexNum(box ComplexNumber::Polar(z)) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + z.to_cartesian();
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            z.to_cartesian(),
                                                        );
                                                    }
                                                };
                                            }
                                            Expr::Constant(c) => {
                                                match var_pows_coeff.get_mut(&(
                                                    x_pow.0,
                                                    f64::from(x_pow.1.clone()).to_bits(),
                                                )) {
                                                    Some(coeff) => {
                                                        *coeff = coeff.clone() + ComplexNumCartesianForm::create_cartesian_complex_num_simple(c.clone(),T::from(0.0));
                                                    }
                                                    None => {
                                                        var_pows_coeff.insert(
                                                            (
                                                                x_pow.0,
                                                                f64::from(x_pow.1.clone())
                                                                    .to_bits(),
                                                            ),
                                                            ComplexNumCartesianForm::create_cartesian_complex_num_simple(c.clone(),T::from(0.0)),
                                                        );
                                                    }
                                                };
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            /* end of code that does ax^n+bx^n=(a+b)x^n*/

                            // check for ax+bx=(a+b)x
                        }
                        if let Expr::Constant(ref c) = x[i] {
                            constants_count += 1;
                            constants_exist = true;
                            constants_sum = constants_sum + c.clone();
                        }
                        if let Expr::ComplexNum(ref c_cont) = x[i] {
                            complex_const_count += 1;
                            complex_const_exist = true;
                            match *c_cont.to_owned() {
                                ComplexNumber::Polar(c) => {
                                    complex_const_sum = complex_const_sum + c.to_cartesian();
                                }
                                ComplexNumber::Cartesian(c) => {
                                    complex_const_sum = complex_const_sum + c.clone();
                                }
                            }
                        }
                    }
                    if constants_count == x.len() {
                        *self = Expr::Constant(constants_sum);
                        return;
                    }
                    if complex_const_count == x.len() {
                        *self =
                            Expr::ComplexNum(Box::new(ComplexNumber::Cartesian(complex_const_sum)));
                        return;
                    }
                    if complex_const_exist {
                        x.retain(|addend| addend.check_if_complex_constant() == false);
                        if !(complex_const_sum.real_part == T::from(0.0)
                            && complex_const_sum.imaginary_part == T::from(0.0))
                        {
                            x.push(Expr::ComplexNum(Box::new(ComplexNumber::Cartesian(
                                complex_const_sum,
                            ))));
                        }
                    }

                    if constants_exist {
                        x.retain(|addend| addend.check_if_constant() == false);
                        if constants_sum != (0.0).into() {
                            x.push(Expr::Constant(constants_sum));
                        }
                    }
                    x.retain(|addend| addend.check_if_is_sum() == false);

                    for var_pow in var_pows_coeff.keys() {
                        let coeff = var_pows_coeff.get(var_pow).unwrap().clone();
                        x.retain(|addend| {
                            if let Expr::Operation(box some_op) = addend {
                                if let Operation::Mul(nested_prod) = some_op {
                                    if nested_prod.len() == 2 {
                                        if (nested_prod[0].check_if_complex_constant()
                                            || nested_prod[0].check_if_constant())
                                            && nested_prod[1]
                                                .check_if_constant_power_of_x()
                                                .is_some()
                                        {
                                            let x_pow = nested_prod[1]
                                                .check_if_constant_power_of_x()
                                                .unwrap();
                                            return !(x_pow.0 == var_pow.0
                                                && f64::from(x_pow.1).to_bits() == var_pow.1);
                                        }

                                        if (nested_prod[1].check_if_complex_constant()
                                            || nested_prod[1].check_if_constant())
                                            && nested_prod[0]
                                                .check_if_constant_power_of_x()
                                                .is_some()
                                        {
                                            let x_pow = nested_prod[0]
                                                .check_if_constant_power_of_x()
                                                .unwrap();
                                            return !(x_pow.0 == var_pow.0
                                                && f64::from(x_pow.1).to_bits() == var_pow.1);
                                        }
                                        if (nested_prod[1].check_if_complex_constant()
                                            || nested_prod[1].check_if_constant())
                                            && nested_prod[0].check_if_variable()
                                        {
                                            return !(nested_prod[0]
                                                .extract_if_variable()
                                                .unwrap()
                                                == var_pow.0
                                                && f64::from_bits(var_pow.1) == 1.0);
                                        }
                                        if (nested_prod[0].check_if_complex_constant()
                                            || nested_prod[0].check_if_constant())
                                            && nested_prod[1].check_if_variable()
                                        {
                                            return !(nested_prod[1]
                                                .extract_if_variable()
                                                .unwrap()
                                                == var_pow.0
                                                && f64::from_bits(var_pow.1) == 1.0);
                                        }
                                    }
                                }
                            }
                            return true;
                        });
                        // NOTE TO DOGGO (ME YAHAHA)
                        // THE BELOW SECTION CAN BE UNCOMMENTED, BUT ONLY AFTER I IMPLEMENT THE
                        // FOLLOWING: USE RETAIN FUNCTION TO REMOVE ALL ADDENDS WITH THE X^N
                        // CURRENTLY BEING LOOKED AT WITH VAR_POW
                        // AND THEN I WILL BE DONE WITH AX^R+BX^R=(A+B)X^R
                        if f64::from_bits(var_pow.1) != 1.0f64 {
                            x.push(Expr::Operation(Box::new(Operation::Mul(vec![
                                Expr::ComplexNum(Box::new(ComplexNumber::Cartesian(coeff))),
                                Expr::Operation(Box::new(Operation::Pow((
                                    Expr::Variable(var_pow.0),
                                    Expr::Constant(T::from(f64::from_bits(var_pow.1))),
                                )))),
                            ]))));
                        } else {
                            x.push(Expr::Operation(Box::new(Operation::Mul(vec![
                                Expr::ComplexNum(Box::new(ComplexNumber::Cartesian(coeff))),
                                Expr::Variable(var_pow.0),
                            ]))));
                        }
                    }

                    x.append(&mut nested_addends);
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
                    if b.check_if_complex_constant() {
                        let reciprocal_of_b: Self = match &b {
                            Expr::Constant(x) => Expr::Constant(T::from(1.0) / x.clone()),
                            Expr::ComplexNum(z) => match *z.to_owned() {
                                ComplexNumber::Cartesian(z_cartesian) => {
                                    Expr::ComplexNum(Box::new(ComplexNumber::Polar(
                                        ComplexNumPolarForm {
                                            modulus: T::from(1.0),
                                            phase: T::from(0.0),
                                        } / z_cartesian.to_polar(),
                                    )))
                                }
                                ComplexNumber::Polar(z_polar) => {
                                    Expr::ComplexNum(Box::new(ComplexNumber::Polar(
                                        ComplexNumPolarForm {
                                            modulus: T::from(1.0),
                                            phase: T::from(0.0),
                                        } / z_polar,
                                    )))
                                }
                            },
                            _ => panic!(),
                        };
                        *self = Expr::create_mul(vec![a, reciprocal_of_b]);
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
                    let mut constants_sum: ComplexNumPolarForm<T> = ComplexNumPolarForm {
                        modulus: T::from(1.0),
                        phase: T::from(0.0),
                    };
                    let mut vars_count: HashMap<char, T> = HashMap::new();
                    let mut nested_factors: Vec<Expr<T>> = vec![];
                    let mut exp_arg_addends: Vec<Expr<T>> = vec![];
                    //for i in dd
                    //check if any factor is equal to 0, set the whole thing to 0 if so
                    for i in 0..x.len() {
                        let pow_of_variable = x[i].check_if_constant_power_of_x();
                        //if pow_of_variable == None {
                        x[i].simplify_with_options(cheese_complex_nums_to_constants);
                        //}
                        if x[i].check_if_zero() {
                            *self = Expr::Constant((0.0).into());
                            return;
                        }
                        if let Expr::Operation(box some_op) = &x[i] {
                            if let Operation::Mul(nested_product) = some_op {
                                for nested_factor in nested_product {
                                    nested_factors.push(nested_factor.clone());
                                }
                            }
                            if let Operation::Exp(argument) = some_op {
                                exp_arg_addends.push(argument.clone());
                            }
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
                            constants_sum = constants_sum
                                * ComplexNumPolarForm {
                                    modulus: c.clone(),
                                    phase: T::from(0.0),
                                };
                        }
                        if let Expr::ComplexNum(ref c_wrapped) = x[i] {
                            constants_count += 1;
                            constants_exist = true;
                            match &**c_wrapped {
                                ComplexNumber::Cartesian(c) => {
                                    constants_sum = constants_sum * c.clone().to_polar();
                                }
                                ComplexNumber::Polar(c) => {
                                    constants_sum = constants_sum * c.clone();
                                }
                            }
                        }
                        res_factors.push(x[i].clone());
                    }
                    if constants_count == x.len() {
                        *self = Expr::ComplexNum(Box::new(ComplexNumber::Polar(constants_sum)));
                        return;
                    }

                    if exp_arg_addends.len() > 0 {
                        let mut exp_arg_sum: Expr<T> =
                            Expr::Operation(Box::new(Operation::Add(exp_arg_addends)));
                        exp_arg_sum.simplify_with_options(cheese_complex_nums_to_constants);
                        res_factors.retain(|factor| factor.check_if_exp() == false);
                        res_factors.push(Expr::Operation(Box::new(Operation::Exp(exp_arg_sum))));
                    }

                    if constants_exist {
                        res_factors.retain(|addend| {
                            addend.check_if_constant() == false
                                && addend.check_if_complex_constant() == false
                        });
                        if !(constants_sum.phase == T::from(0.0)) {
                            res_factors.push(Expr::ComplexNum(Box::new(ComplexNumber::Polar(
                                constants_sum,
                            ))));
                        } else if constants_sum.modulus != T::from(1.0)
                            && constants_sum.phase == T::from(0.0)
                        {
                            res_factors.push(Expr::Constant(constants_sum.modulus));
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
                    res_factors.retain(|factor| factor.check_if_is_mul() == false);
                    res_factors.append(&mut nested_factors);
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
                Operation::Exp(mut argument) => {
                    argument.simplify();
                    *self = Expr::Operation(Box::new(Operation::Exp(argument)));
                }
                _ => {}
            },
        }
    }
    pub fn evaluate_expr(&self, variable_values: &HashMap<char, T>) -> T
    where
        T: TrigOps,
    {
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
                        res = res + x[i].evaluate_expr(variable_values);
                    }
                    return res;
                }
                Operation::Mul(x) => {
                    let mut res: T = (1.0).into();
                    for i in 0..x.len() {
                        res = res * x[i].evaluate_expr(variable_values);
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
                    return (x.evaluate_expr(variable_values)).sin();
                }
                Operation::Trig(TrigOp::Cos(x)) => {
                    return (x.evaluate_expr(variable_values)).cos();
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
impl<T: std::clone::Clone + std::cmp::PartialEq + TrigOps + std::cmp::PartialEq> PartialEq
    for Expr<T>
{
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
fn turn_complex_exps_into_trigs<T>(x: &mut Vec<Expr<T>>)
where
    T: std::clone::Clone
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
    let mut trig_list: Vec<Expr<T>> = vec![];
    let mut arg_list: Vec<(T, T, usize, usize)> = vec![];
    let mut sine_list: Vec<(T, T, usize, usize)> = vec![];
    let mut indices_to_remove: Vec<usize> = vec![];
    for i in 0..x.len() {
        if let Expr::Operation(box Operation::Mul(outer_prod)) = &x[i] {
            match (&outer_prod[0], &outer_prod[1]) {
                (
                    Expr::Operation(box Operation::Exp(Expr::Operation(box Operation::Mul(
                        arg_prod,
                    )))),
                    Expr::Constant(c),
                )
                | (
                    Expr::Constant(c),
                    Expr::Operation(box Operation::Exp(Expr::Operation(box Operation::Mul(
                        arg_prod,
                    )))),
                ) => {
                    if arg_prod.len() == 2 {
                        match (&arg_prod[0], &arg_prod[1]) {
                            (Expr::Variable(var_c), Expr::ComplexNum(z_cont))
                            | (Expr::ComplexNum(z_cont), Expr::Variable(var_c)) => {
                                match *z_cont.to_owned() {
                                    ComplexNumber::Polar(z) => {
                                        for j in 0..arg_list.len() {
                                            if arg_list[j].1
                                                == T::from(-1.0f64)
                                                    * z.to_cartesian().imaginary_part
                                            {
                                                if arg_list[j].0 == c.clone() {
                                                    arg_list[j].3 = i;
                                                    trig_list.push(make_const_multiple_of_cosine(
                                                        Expr::Constant(
                                                            T::from(2.0) * arg_list[j].0.clone(),
                                                        ),
                                                        arg_list[j].1.clone(),
                                                        *var_c,
                                                    ));
                                                    indices_to_remove.push(arg_list[j].3);
                                                    indices_to_remove.push(arg_list[j].2);
                                                    arg_list.remove(j);
                                                    break;
                                                }
                                            }
                                        }
                                        arg_list.push((
                                            c.clone(),
                                            z.to_cartesian().imaginary_part,
                                            i,
                                            i,
                                        ))
                                    }
                                    ComplexNumber::Cartesian(z) => {
                                        for j in 0..arg_list.len() {
                                            if arg_list[j].1
                                                == T::from(-1.0f64) * z.imaginary_part.clone()
                                            {
                                                if arg_list[j].0 == c.clone() {
                                                    arg_list[j].3 = i;
                                                    trig_list.push(make_const_multiple_of_cosine(
                                                        Expr::Constant(
                                                            T::from(2.0) * arg_list[j].0.clone(),
                                                        ),
                                                        arg_list[j].1.clone(),
                                                        *var_c,
                                                    ));
                                                    indices_to_remove.push(arg_list[j].3);
                                                    indices_to_remove.push(arg_list[j].2);
                                                    arg_list.remove(j);
                                                    break;
                                                }
                                            }
                                        }
                                        arg_list.push((c.clone(), z.imaginary_part.clone(), i, i))
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                (
                    Expr::Operation(box Operation::Exp(Expr::Operation(box Operation::Mul(
                        arg_prod,
                    )))),
                    Expr::ComplexNum(c),
                )
                | (
                    Expr::ComplexNum(c),
                    Expr::Operation(box Operation::Exp(Expr::Operation(box Operation::Mul(
                        arg_prod,
                    )))),
                ) => {
                    let cf: ComplexNumCartesianForm<T> = match *c.to_owned() {
                        ComplexNumber::Polar(cp) => cp.to_cartesian(),
                        ComplexNumber::Cartesian(cc) => cc,
                    };
                    if arg_prod.len() == 2 {
                        match (&arg_prod[0], &arg_prod[1]) {
                            (Expr::Variable(var_c), Expr::ComplexNum(z_cont))
                            | (Expr::ComplexNum(z_cont), Expr::Variable(var_c)) => {
                                match *z_cont.to_owned() {
                                    ComplexNumber::Polar(z) => {
                                        for j in 0..sine_list.len() {
                                            if sine_list[j].1
                                                == T::from(-1.0f64)
                                                    * z.to_cartesian().imaginary_part
                                            {
                                                if sine_list[j].0
                                                    == (T::from(-1.0) * cf.imaginary_part.clone())
                                                {
                                                    sine_list[j].3 = i;
                                                    trig_list.push(make_const_multiple_of_sine(
                                                        Expr::Constant(
                                                            T::from(-2.0) * arg_list[j].0.clone(),
                                                        ),
                                                        arg_list[j].1.clone(),
                                                        *var_c,
                                                    ));
                                                    indices_to_remove.push(sine_list[j].3);
                                                    indices_to_remove.push(sine_list[j].2);
                                                    sine_list.remove(j);
                                                    break;
                                                }
                                            }
                                        }
                                        arg_list.push((
                                            cf.imaginary_part.clone(),
                                            z.to_cartesian().imaginary_part,
                                            i,
                                            i,
                                        ))
                                    }
                                    ComplexNumber::Cartesian(z) => {
                                        for j in 0..sine_list.len() {
                                            if sine_list[j].1
                                                == T::from(-1.0f64) * z.imaginary_part.clone()
                                            {
                                                if sine_list[j].0
                                                    == (T::from(-1.0) * cf.imaginary_part.clone())
                                                {
                                                    sine_list[j].3 = i;
                                                    trig_list.push(make_const_multiple_of_sine(
                                                        Expr::Constant(
                                                            T::from(-2.0) * arg_list[j].0.clone(),
                                                        ),
                                                        arg_list[j].1.clone(),
                                                        *var_c,
                                                    ));
                                                    indices_to_remove.push(sine_list[j].3);
                                                    indices_to_remove.push(sine_list[j].2);
                                                    sine_list.remove(j);
                                                    break;
                                                }
                                            }
                                        }
                                        arg_list.push((
                                            cf.imaginary_part.clone(),
                                            z.imaginary_part.clone(),
                                            i,
                                            i,
                                        ))
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
    //println!("yahh {:?}",arg_list);
    /*    for arg in arg_list {
        indices_to_remove.push(arg.2);
    }
    for arg in sine_list {
        indices_to_remove.push(arg.3);
        indices_to_remove.push(arg.2);
    }*/
    //println!("{:?}",indices_to_remove);
    //remove_all_duplicates(&mut indices_to_remove); what's wrong with this line? must check why
    //indices_to_remove has duplicates even when it shouldn't
    remove_indices(x, indices_to_remove);
    for trig_elem in trig_list {
        x.push(trig_elem);
    }
}
fn make_const_multiple_of_cosine<T>(sine_coeff: Expr<T>, var_coeff: T, variable: char) -> Expr<T>
where
    T: std::clone::Clone
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
    return Expr::Operation(Box::new(Operation::Mul(vec![
        sine_coeff,
        Expr::Operation(Box::new(Operation::Trig(TrigOp::Cos(Expr::Operation(
            Box::new(Operation::Mul(vec![
                Expr::Constant(var_coeff),
                Expr::Variable(variable),
            ])),
        ))))),
    ])));
}

fn make_const_multiple_of_sine<T>(sine_coeff: Expr<T>, var_coeff: T, variable: char) -> Expr<T>
where
    T: std::clone::Clone
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
    return Expr::Operation(Box::new(Operation::Mul(vec![
        sine_coeff,
        Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(Expr::Operation(
            Box::new(Operation::Mul(vec![
                Expr::Constant(var_coeff),
                Expr::Variable(variable),
            ])),
        ))))),
    ])));
}
pub fn remove_indices<T>(vec: &mut Vec<Expr<T>>, indices: Vec<usize>)
where
    T: Clone + TrigOps,
{
    let mut sorted = indices.clone();
    sorted.sort_unstable_by(|a, b| b.cmp(a));

    for &i in &sorted {
        if i < vec.len() {
            vec.remove(i);
        }
    }
}
fn _remove_all_duplicates<T: Eq + std::hash::Hash + Clone>(v: &mut Vec<T>) {
    let mut counts = HashMap::new();
    for x in v.iter() {
        *counts.entry(x.clone()).or_insert(0) += 1;
    }
    v.retain(|x| counts[x] == 1);
}
