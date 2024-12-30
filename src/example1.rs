use crate::ad::*;

fn f<T: VectorSpace>(t: &Dual<T>) -> Dual<T> {
    t.powi(2) + t + Dual::constant(1.0)
}

fn df(t: f64) -> f64 {
    let res = f(&Dual::new(t, 1.0));
    res.tangent
}

pub fn run() {
    let res = f(&Dual::new(10.0, 1.0));
    println!("{res:?}");

    let res = df(10.0);
    println!("{res:?}");
}
