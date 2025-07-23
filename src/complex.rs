use crate::structs::{ComplexNumber, Expr};
#[derive(Debug, Clone, Copy)]
pub struct ComplexNumCartesianForm<T: Clone> {
    pub real_part: T,
    pub imaginary_part: T,
}
#[derive(Debug, Clone, Copy)]
pub struct ComplexNumPolarForm<T: Clone> {
    pub modulus: T,
    pub phase: T,
}
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
    > ComplexNumPolarForm<T>
where
    f64: From<T>,
{
    pub fn to_cartesian(&self) -> ComplexNumCartesianForm<T> {
        ComplexNumCartesianForm {
            real_part: self.modulus.clone() * T::from(f64::from(self.phase.clone()).cos()),
            imaginary_part: self.modulus.clone() * T::from(f64::from(self.phase.clone()).sin()),
        }
    }
}
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
    > ComplexNumCartesianForm<T>
where
    f64: From<T>,
{
    pub fn create_cartesian_complex_num(a: T, b: T) -> Expr<T> {
        Expr::ComplexNum(Box::new(ComplexNumber::Cartesian(
            ComplexNumCartesianForm {
                real_part: a,
                imaginary_part: b,
            },
        )))
    }

    pub fn to_polar(&self) -> ComplexNumPolarForm<T> {
        let modulus = T::from(
            f64::from(
                T::from(f64::from(self.real_part.clone()).powi(2))
                    + T::from(f64::from(self.imaginary_part.clone()).powi(2)),
            )
            .sqrt(),
        );
        let phase = T::from(
            f64::from(self.imaginary_part.clone()).atan2(f64::from(self.real_part.clone())),
        ); // handles full circle correctly
        ComplexNumPolarForm { modulus, phase }
    }
    //pub fn modulus
}
impl<T: std::clone::Clone + std::ops::Add<Output = T>> std::ops::Add
    for ComplexNumCartesianForm<T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real_part: self.real_part + other.real_part,
            imaginary_part: self.imaginary_part + other.imaginary_part,
        }
    }
}
impl<T: std::clone::Clone + std::ops::Mul<Output = T> + std::ops::Sub<Output = T>> std::ops::Mul
    for ComplexNumCartesianForm<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            real_part: self.real_part.clone() * rhs.real_part.clone()
                - self.imaginary_part.clone() * rhs.imaginary_part.clone(),
            imaginary_part: self.real_part.clone() * rhs.imaginary_part.clone()
                - self.imaginary_part.clone() * rhs.real_part.clone(),
        }
    }
}

impl<T: std::clone::Clone + std::ops::Mul<Output = T> + std::ops::Add<Output = T>> std::ops::Mul
    for ComplexNumPolarForm<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            modulus: self.modulus * rhs.modulus,
            phase: self.phase + rhs.phase,
        }
    }
}
impl<T: std::clone::Clone + std::ops::Div<Output = T> + std::ops::Sub<Output = T>> std::ops::Div
    for ComplexNumPolarForm<T>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            modulus: self.modulus / rhs.modulus,
            phase: self.phase - rhs.phase,
        }
    }
}
