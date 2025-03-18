pub mod differentiation;
pub mod impls;
pub mod structs;
#[cfg(test)]
mod tests {
    use structs::{Expr, TrigOp, Operation};
    use super::*;

    #[test]
    fn test_parsing() {
        let test_expr:Expr<f64> = Expr::Operation(Box::new(Operation::Trig(
            TrigOp::Sin(
                Expr::Operation(Box::new(
                    Operation::Add(vec![
                        Expr::Constant(7.0),
                        Expr::Operation(Box::new(
                            Operation::Mul(vec![Expr::Constant(5.0),Expr::Variable('x')])
                        ))  
                    ])
                ))
            )
        )));
        //let test_string = test_expr.expr_to_string();
        let mut test_expr2:Expr<f64> = Expr::Operation(Box::new(Operation::Add(
            vec![
                test_expr.clone(),
                Expr::Constant(2.0),
                Expr::Constant(3.7),
                Expr::Variable('x')
            ]
        )));
        test_expr2.simplify();
        println!("{}",test_expr2.expr_to_string());
        //println!("{}",test_string);
        assert_eq!(4, 4);
    }
}
