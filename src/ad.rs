use std::{
    fmt::Display,
    ops::{Add, Mul},
};

#[derive(Debug)]
pub struct Dual<T> {
    pub primal: f64,
    pub tangent: T,
}

impl<T> Display for Dual<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.primal, self.tangent,)
    }
}

pub trait VectorSpace {
    fn zero() -> Self;
    fn add(&self, rhs: &Self) -> Self;
    fn scale(&self, factor: f64) -> Self;
}

impl VectorSpace for f64 {
    fn zero() -> Self {
        0.0
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn scale(&self, factor: f64) -> Self {
        self * factor
    }
}

impl<T: VectorSpace> Dual<T> {
    pub fn constant(primal: f64) -> Self {
        Dual {
            primal,
            tangent: T::zero(),
        }
    }

    pub fn new(primal: f64, tangent: T) -> Self {
        Dual { primal, tangent }
    }

    fn chain(&self, primal: f64, factor: f64) -> Self {
        Dual::new(primal, self.tangent.scale(factor))
    }

    pub fn sin(&self) -> Self {
        self.chain(self.primal.sin(), self.primal.cos())
    }

    pub fn cos(&self) -> Self {
        self.chain(self.primal.cos(), -self.primal.sin())
    }

    pub fn powi(&self, n: i32) -> Self {
        self.chain(self.primal.powi(n), f64::from(n) * self.primal.powi(n - 1))
    }

    fn add_impl(&self, rhs: &Dual<T>) -> Dual<T> {
        Dual::new(self.primal + rhs.primal, self.tangent.add(&rhs.tangent))
    }

    fn mul_impl(&self, rhs: &Dual<T>) -> Dual<T> {
        Dual::new(
            self.primal * rhs.primal,
            rhs.tangent
                .scale(self.primal)
                .add(&self.tangent.scale(rhs.primal)),
        )
    }
}

impl<T: VectorSpace> Add for Dual<T> {
    type Output = Dual<T>;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_impl(&rhs)
    }
}

impl<T: VectorSpace> Add for &Dual<T> {
    type Output = Dual<T>;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_impl(rhs)
    }
}

impl<T: VectorSpace> Add<&Dual<T>> for Dual<T> {
    type Output = Dual<T>;

    fn add(self, rhs: &Dual<T>) -> Self::Output {
        self.add_impl(rhs)
    }
}

impl<T: VectorSpace> Add<Dual<T>> for &Dual<T> {
    type Output = Dual<T>;

    fn add(self, rhs: Dual<T>) -> Self::Output {
        self.add_impl(&rhs)
    }
}

impl<T: VectorSpace> Mul for Dual<T> {
    type Output = Dual<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_impl(&rhs)
    }
}

impl<T: VectorSpace> Mul for &Dual<T> {
    type Output = Dual<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_impl(rhs)
    }
}

impl<T: VectorSpace> Mul<&Dual<T>> for Dual<T> {
    type Output = Dual<T>;

    fn mul(self, rhs: &Dual<T>) -> Self::Output {
        self.mul_impl(rhs)
    }
}

impl<T: VectorSpace> Mul<Dual<T>> for &Dual<T> {
    type Output = Dual<T>;

    fn mul(self, rhs: Dual<T>) -> Self::Output {
        self.mul_impl(&rhs)
    }
}
