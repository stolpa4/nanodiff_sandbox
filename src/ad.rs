#[derive(Debug)]
pub struct Dual<T> {
    pub primal: f64,
    pub tangent: T,
}

pub trait VectorSpace {
    fn zero() -> Self;
    fn add(&self, rhs: &Self) -> Self;
    fn scale(&self, factor: f64) -> Self;
}
