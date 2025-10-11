#![feature(box_patterns)]
#![feature(let_chains)]
pub mod complex;
pub mod differentiation;
pub mod impls;
pub mod integration;
pub mod series;
pub mod structs;
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::f64;

    // use crate::complex::ComplexNumCartesianForm;

    use crate::complex::ComplexNumCartesianForm;

    use super::*;
    //use complex::*;
    use structs::ComplexNumber;
    use structs::{Expr, Operation, TrigOp};
    #[test]
    fn having_fun() {
        let test_product = Expr::Operation(Box::new(Operation::Mul(vec![
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Cos(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(12.0)],
            ))))),
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Cos(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(3.0)],
            ))))),
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(2.0)],
            ))))),
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Cos(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(7.0)],
            ))))),
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(5.0)],
            ))))),
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(34.0)],
            ))))),
            /*Expr::Operation(Box::new(Operation::Trig(TrigOp::Cos(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(4.0)],
            ))))),
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Cos(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(3.0)],
            ))))),
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(2.0)],
            ))))),*/
        ])));
        let mut int_res = test_product.integrate('x');
        int_res.simplify_with_options(true);
        int_res.simplify_with_options(true);
        int_res.simplify_complex_exps();
        println!("integral result is {:?}", int_res.expr_to_string());
        //let mut xval1: HashMap<char, ComplexNumber<f64>> = HashMap::new();
        let mut xval1: HashMap<char, f64> = HashMap::new();
        xval1.insert('x', 1.0f64);
        let mut xval2: HashMap<char, f64> = HashMap::new();
        xval2.insert('x', 10.0f64);
        println!(
            "{}",
            int_res.evaluate_expr(&xval2) - int_res.evaluate_expr(&xval1)
        );
        /*xval1.insert(
            'x',
            ComplexNumber::Cartesian(
                ComplexNumCartesianForm::create_cartesian_complex_num_simple(0.0f64, 0.0f64),
            ),
        );
        let mut xval2: HashMap<char, ComplexNumber<f64>> = HashMap::new();
        xval2.insert(
            'x',
            ComplexNumber::Cartesian(
                ComplexNumCartesianForm::create_cartesian_complex_num_simple(10.0f64, 0.0f64),
            ),
        );
        println!("{:?}", int_res.evaluate_complex_expr(&xval2));
        println!("{:?}", int_res.evaluate_complex_expr(&xval1));*/
    }
    #[test]
    fn test_complex_simp() {
        let test_arg: Expr<f64> = Expr::Operation(Box::new(Operation::Add(vec![
            Expr::Operation(Box::new(Operation::Mul(vec![
                Expr::ComplexNum(Box::new(ComplexNumber::Polar(
                    complex::ComplexNumPolarForm {
                        modulus: 2.0,
                        phase: f64::consts::FRAC_PI_2,
                    },
                ))),
                Expr::Variable('x'),
            ]))),
            Expr::Operation(Box::new(Operation::Mul(vec![
                Expr::ComplexNum(Box::new(ComplexNumber::Polar(
                    complex::ComplexNumPolarForm {
                        modulus: 3.0,
                        phase: f64::consts::FRAC_PI_2,
                    },
                ))),
                Expr::Variable('x'),
            ]))),
        ])));
        let mut test_func: Expr<f64> = Expr::Operation(Box::new(Operation::Exp(test_arg)));
        test_func.simplify();
        println!("{:#?}", test_func);
    }
    #[test]
    fn test_complex() {
        let test_exp: Expr<f64> = Expr::Operation(Box::new(Operation::Exp(Expr::Variable('x'))));
        let test_funca: Expr<f64> = Expr::Operation(Box::new(Operation::Mul(vec![
            /*Expr::Operation(Box::new(Operation::Mul(vec![
                Expr::Variable('x'),
                Expr::Constant(1.0),
                Expr::Constant(2.0),
            ]))),*/
            Expr::Variable('x'),
            Expr::ComplexNum(Box::new(ComplexNumber::Cartesian(
                complex::ComplexNumCartesianForm {
                    real_part: -2.0,
                    imaginary_part: -1.0,
                },
            ))),
            Expr::ComplexNum(Box::new(ComplexNumber::Polar(
                complex::ComplexNumPolarForm {
                    modulus: 2.0,
                    phase: f64::consts::FRAC_PI_2,
                },
            ))),
        ])));
        let test_func: Expr<f64> =
            Expr::Operation(Box::new(Operation::Div((test_funca, Expr::Variable('y')))));
        let mut xval = HashMap::new();
        xval.insert(
            'x',
            ComplexNumber::Polar(complex::ComplexNumPolarForm {
                modulus: 4.0,
                phase: f64::consts::FRAC_PI_4,
            }),
        );
        xval.insert(
            'y',
            ComplexNumber::Cartesian(complex::ComplexNumCartesianForm {
                real_part: 2.0,
                imaginary_part: -5.0,
            }),
        );
        println!("val is {:?}", test_func.evaluate_complex_expr(&xval));
        println!("exp val is {:?}", test_exp.evaluate_complex_expr(&xval));
    }
    #[test]
    fn test_simplification() {
        let mut test_func = Expr::Operation(Box::new(Operation::Add(vec![
            Expr::Operation(Box::new(Operation::Mul(vec![
                Expr::Variable('x'),
                Expr::Constant(1.0),
                Expr::Constant(2.0),
            ]))),
            Expr::Operation(Box::new(Operation::Pow((
                Expr::Variable('x'),
                Expr::Constant(2.0f64),
            )))),
            Expr::Operation(Box::new(Operation::Add(vec![
                Expr::Variable('a'),
                Expr::Constant(2.0),
            ]))),
        ])));
        test_func.simplify();
        println!("yahah {}", test_func.expr_to_string());
        let mut test_func2 = Expr::Operation(Box::new(Operation::Mul(vec![
            Expr::Variable('x'),
            Expr::Operation(Box::new(Operation::Pow((
                Expr::Variable('x'),
                Expr::Constant(2.0f64),
            )))),
            Expr::Constant(2.0),
            Expr::Operation(Box::new(Operation::Pow((
                Expr::Variable('x'),
                Expr::Constant(2.0f64),
            )))),
        ])));
        test_func2.simplify();
        println!("yahah {}", test_func2.expr_to_string());
    }
    #[test]
    fn test_distributive() {
        let mut t: Expr<f64> = Expr::Operation(Box::new(Operation::Add(vec![
            Expr::create_mul(vec![
                Expr::Operation(Box::new(Operation::Pow((
                    Expr::Variable('x'),
                    Expr::Constant(2.1),
                )))),
                ComplexNumCartesianForm::create_cartesian_complex_num(1.3, 2.0),
            ]),
            Expr::create_mul(vec![
                Expr::Operation(Box::new(Operation::Pow((
                    Expr::Variable('x'),
                    Expr::Constant(2.1),
                )))),
                Expr::Constant(2.1),
            ]),
        ])));
        t.simplify();
        println!("{}", t.expr_to_string());
    }
    #[test]
    fn test_integration() {
        /*let test_product = Expr::Operation(Box::new(Operation::Mul(vec![
            Expr::Constant(2.0),
            Expr::Operation(Box::new(Operation::Add(vec![
                Expr::Variable('x'),
                Expr::Constant(1.0),
            ]))),
            Expr::Operation(Box::new(Operation::Pow((
                Expr::Variable('x'),
                Expr::Constant(2.0f64),
            )))),
            Expr::Operation(Box::new(Operation::Add(vec![
                Expr::Variable('a'),
                Expr::Constant(2.0),
            ]))),
        ])));*/

        let test_product = Expr::Operation(Box::new(Operation::Mul(vec![
            //Expr::Constant(2.0),
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Cos(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(3.0)],
            ))))),
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(Expr::create_mul(
                vec![Expr::Variable('x'), Expr::Constant(2.0)],
            ))))),
        ])));
        let mut int_res = test_product.integrate('x');
        int_res.simplify_with_options(true);
        int_res.simplify_with_options(true);
        //println!("{:#?}", int_res);
        println!("integral result is {:?}", int_res.expr_to_string());
        int_res.simplify_complex_exps();
        println!("{:#?}", int_res);
        println!("integral result is {:?}", int_res.expr_to_string());
    }
    #[test]
    fn test_legendre() {
        println!(
            "{}",
            series::factorial(3, &mut std::collections::HashMap::new())
        );
        let l: Vec<Vec<Expr<f64>>> = series::generate_associated_legendre_polynomials(4, 4);
        let mut a = l[4][4].clone();
        a.simplify();
        let mut xval = HashMap::new();
        xval.insert('x', 0.3);
        println!("val is {}", a.evaluate_expr(&xval));
    }
    #[test]
    fn test_parsing() {
        let test_expr3: Expr<f64> = Expr::Operation(Box::new(Operation::Pow((
            Expr::Operation(Box::new(Operation::Add(vec![
                Expr::Constant(1.0f64),
                Expr::Operation(Box::new(Operation::Mul(vec![
                    Expr::Constant(-1.0f64),
                    Expr::Operation(Box::new(Operation::Pow((
                        Expr::Variable('x'),
                        Expr::Constant(2.0f64),
                    )))),
                ]))),
            ]))),
            Expr::Constant((2 as f64) / 2.0),
        ))));
        println!("{}", test_expr3.expr_to_string());
        let test_expr: Expr<f64> = Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(
            Expr::Operation(Box::new(Operation::Add(vec![
                Expr::Constant(7.0),
                Expr::Operation(Box::new(Operation::Mul(vec![
                    Expr::Constant(5.0),
                    Expr::Variable('x'),
                ]))),
            ]))),
        ))));
        let _test_expr2: Expr<f64> = Expr::Operation(Box::new(Operation::Div((
            Expr::Operation(Box::new(Operation::Trig(TrigOp::Sin(Expr::Operation(
                Box::new(Operation::Add(vec![
                    Expr::Operation(Box::new(Operation::Pow((
                        Expr::Variable('x'),
                        Expr::Constant(3.0),
                    )))),
                    Expr::Constant(7.0),
                ])),
            ))))),
            Expr::Operation(Box::new(Operation::Log(Expr::Variable('y')))),
        ))));
        //let test_string = test_expr.expr_to_string();
        let mut test_expr2: Expr<f64> = Expr::Operation(Box::new(Operation::Add(vec![
            test_expr.clone(),
            Expr::Constant(2.0),
            Expr::Constant(3.7),
            Expr::Variable('x'),
        ])));
        test_expr2.simplify();
        println!("{}", test_expr2.expr_to_string());
        //println!("{}",test_string);
        assert_eq!(4, 4);
    }
}
