use crate::structs::{Expr, HyperbolicOp, Operation, TrigOp};
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
    pub fn expr_to_string(&self) -> String {
        match self {
            Expr::ComplexNum(_z) => {
                todo!()
            }
            Expr::Constant(x) => {
                return f64::from(x.clone()).to_string();
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
    fn check_if_constant(&self) -> bool {
        /*if let Expr::Constant(x) = self {
            true
        } else {
            false
        }*/
        match self {
            Expr::Constant(_x) => {
                return true;
            }
            _ => false,
        }
    }
    /*fn check_if_operation(&self) -> bool {
        if let Expr::Operation(_x) = self {
            return true;
        }
        return false;
    }*/
    fn check_if_variable(&self) -> bool {
        if let Expr::Variable(_x) = self {
            return true;
        }
        return false;
    }
    /*fn extract_operation(&self) -> Operation<T> {
        match self {
            Expr::Operation(x) => {
                let x_unboxed = *x.to_owned();
                return x_unboxed;
            }
            _ => {
                panic!("not an operation lol")
            }
        }
    }*/
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
                    let mut res_factors: Vec<Expr<T>> = Vec::new();
                    let mut constants_exist = false;
                    let mut constants_count: usize = 0;
                    let mut constants_sum: T = (1.0).into();
                    let mut vars_count: HashMap<char,usize> = HashMap::new();
                    //for i in dd
                    //check if any factor is equal to 0, set the whole thing to 0 if so
                    for i in 0..x.len() {
                        x[i].simplify();
                        if x[i].check_if_zero() {
                            *self = Expr::Constant((0.0).into());
                            return;
                        }
                        if x[i].check_if_variable(){
                            let var_i = match x[i]{
                                Self::Variable(c)=>c,
                                _=>panic!("expected variable, found something else")
                            };
                            match vars_count.get_mut(&var_i){
                                Some(var_i_count)=>{
                                    *var_i_count+=1;
                                }
                                None=>{
                                    vars_count.insert(var_i, 1);
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
                        res_factors.push(Expr::Constant(constants_sum));
                    }
                    for var_letter in vars_count.keys(){
                        res_factors.retain(|factor| factor.check_if_variable()==false);
                        res_factors.push(Expr::Operation(Box::new(
                            Operation::Pow((
                                Expr::Variable(*var_letter)
                                ,
                                Expr::Constant(T::from(*vars_count.get(var_letter).unwrap() as f64))
                           ))
                        )));
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
            Expr::ComplexNum(_z) => {
                todo!()
            }
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
