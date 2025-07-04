#[derive(Debug, Clone)]
pub struct ComplexNumCartesianForm<T: Clone> {
    pub real_part: T,
    pub imaginary_part: T,
}
#[derive(Debug, Clone)]
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
    > ComplexNumCartesianForm<T>
where
    f64: From<T>,
{
    //pub fn modulus
}
impl<T:std::clone::Clone + std::ops::Add<Output = T>> std::ops::Add for ComplexNumCartesianForm<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real_part: self.real_part + other.real_part,
            imaginary_part: self.imaginary_part + other.imaginary_part,
        }
    }
}
impl<T:std::clone::Clone + std::ops::Mul<Output = T>> std::ops::Mul for ComplexNumCartesianForm<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            real_part: self.real_part * rhs.real_part,
            imaginary_part: self.imaginary_part * rhs.imaginary_part,
        }
    }
}
