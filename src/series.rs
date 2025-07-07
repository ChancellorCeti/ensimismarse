use crate::{
    differentiation::differentiate,
    structs::{Expr, Operation},
};
pub fn create_multipole_expansion<T>(_f: Expr<T>) -> Expr<T>
where
    T: From<f64> + std::clone::Clone,
{
    todo!()
}
pub fn generate_associated_legendre_polynomials<T>(l_max: usize, m_max: usize) -> Vec<Vec<Expr<T>>>
where
    T: From<f64>
        + std::clone::Clone
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>
        + std::cmp::PartialEq
        + std::fmt::Debug,
    f64: From<T>,
{
    let mut factorial_cache = std::collections::HashMap::new();
    let mut res: Vec<Vec<Expr<T>>> = vec![vec![Expr::Constant(T::from(0.0)); m_max + 1]; l_max + 1];
    for l in 0..=l_max {
        for m in 0..=m_max {
            let mut lm_deriv: Expr<T> = Expr::Operation(Box::new(Operation::Pow((
                Expr::Operation(Box::new(Operation::Add(vec![
                    Expr::Operation(Box::new(Operation::Pow((
                        Expr::Variable('x'),
                        Expr::Constant(T::from(2.0f64)),
                    )))),
                    Expr::Constant(T::from(-1.0)),
                ]))),
                Expr::Constant(T::from(l as f64)),
            ))));
            for _i in 1..=(l + m) {
                lm_deriv = differentiate(lm_deriv, 'x');
                lm_deriv.simplify();
            }
            res[l][m] = Expr::Operation(Box::new(Operation::Mul(vec![
                Expr::Constant(T::from(
                    ((-1.0f64).powi(m as i32))
                        / (2f64.powi(l as i32) * factorial(l, &mut factorial_cache)),
                )),
                Expr::Operation(Box::new(Operation::Pow((
                    Expr::Operation(Box::new(Operation::Add(vec![
                        Expr::Constant(T::from(1.0f64)),
                        Expr::Operation(Box::new(Operation::Mul(vec![
                            Expr::Constant(T::from(-1.0f64)),
                            Expr::Operation(Box::new(Operation::Pow((
                                Expr::Variable('x'),
                                Expr::Constant(T::from(2.0f64)),
                            )))),
                        ]))),
                    ]))),
                    Expr::Constant(T::from((m as f64) / 2.0)),
                )))),
                lm_deriv,
            ])))
        }
    }
    res
}

pub fn generate_legendre_polynomials<T>(l_max: usize) -> Vec<Expr<T>>
where
    T: From<f64> + std::clone::Clone,
{
    let mut res: Vec<Expr<T>> = vec![Expr::Constant(T::from(0.0)); l_max + 1];
    res[0] = Expr::Constant(T::from(1.0));
    res[1] = Expr::Variable('x');
    for l in 1..l_max {
        res[l + 1] = Expr::Operation(Box::new(Operation::Add(vec![
            Expr::Operation(Box::new(Operation::Mul(vec![
                Expr::Constant(T::from(
                    (2.0f64 * (l as f64) + 1.0f64) / ((l as f64) + 1.0f64),
                )),
                Expr::Variable('x'),
                res[l].clone(),
            ]))),
            Expr::Operation(Box::new(Operation::Mul(vec![
                Expr::Constant(T::from(-1.0)),
                Expr::Constant(T::from((l as f64) / (l as f64 + 1.0f64))),
                res[l - 1].clone(),
            ]))),
        ])));
    }
    res
}

pub fn double_factorial(n: usize) -> f64 {
    let mut res: f64 = 1.0f64;

    for i in 0..=((n as f64) / 2.0).floor() as usize - 1 {
        res *= n as f64 - 2.0 * i as f64;
    }
    return res;
}

pub fn factorial(n: usize, cache: &mut std::collections::HashMap<usize, f64>) -> f64 {
    if cache.contains_key(&n) {
        return *cache.get(&n).unwrap() as f64;
    }
    cache.insert(1, 1f64);
    let highest_key = *cache.keys().max().unwrap_or(&1);
    let mut res: f64 = *cache.get(&highest_key).unwrap();

    for i in (highest_key + 1)..=n {
        res *= i as f64;
    }
    cache.insert(n, res);

    return res;
}

fn _generate_spherical_harmonics<T>(l_max: usize, m_max: usize) -> Vec<Vec<Expr<T>>>
where
    T: From<f64> + std::clone::Clone,
{
    let mut factorial_cache = std::collections::HashMap::new();
    let mut res: Vec<Vec<Expr<T>>> = vec![vec![Expr::Constant(T::from(0.0)); m_max + 1]; l_max + 1];
    for l in 0..=l_max {
        for m in 0..=m_max {
            res[l][m] = Expr::Operation(Box::new(Operation::Mul(vec![
                Expr::Constant(T::from(
                    (((2.0f64 * (l as f64) + 1.0f64) * factorial(l - m, &mut factorial_cache))
                        / (4.0f64 * std::f64::consts::PI * factorial(l + m, &mut factorial_cache)))
                    .sqrt(),
                )),
                /*to-do tmrw: replace this 0 with e^{im\theta} and also add a factor of
                 * P_l^{m}(\cos{\theta})*/
                Expr::Constant(T::from(1.0)), // implement complex numbers as enum -- complexnum can either be polar form or
                                              // (re,im) -- also note that complexnum needs to be a kind of constant in the enum
                                              // definition somehwere (figure it out idot)
            ])));
        }
    }
    res
}

pub struct Series<T: std::clone::Clone> {
    _term_count: usize,
    _terms: Vec<Expr<T>>,
}
