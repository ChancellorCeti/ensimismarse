pub mod differentiation;
pub mod complex;
pub mod impls;
pub mod structs;
pub mod series;
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use structs::{Expr, Operation, TrigOp};
    #[test]
    fn test_legendre(){
        println!("{}",series::factorial(3,&mut std::collections::HashMap::new()));
        let l:Vec<Vec<Expr<f64>>>= series::generate_associated_legendre_polynomials(40,40);
        let mut a = l[4][4].clone();
        a.simplify();
        let mut xval = HashMap::new();
        xval.insert('x',0.3);
        println!("val is {}",a.evaluate_expr(&xval));
    }
    #[test]
    fn test_parsing() {
        let test_expr3:Expr<f64>=Expr::Operation(Box::new(Operation::Pow((
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
        println!("{}",test_expr3.expr_to_string());
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
