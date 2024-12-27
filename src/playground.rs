fn f(t: f64) -> f64 {
    t.powi(2) + t + 1.0
}

fn df_sym(t: f64) -> f64 {
    2.0 * t + 1.0
}

fn df_num(t: f64, h: f64) -> f64 {
    (f(t + h) - f(t)) / h
}

fn f_ad(t_dual: (f64, f64)) -> (f64, f64) {
    let (primalt, tangentt) = t_dual;
    let (primal0, tangent0) = (primalt.powi(2), 2.0 * primalt * tangentt);
    let (primal1, tangent1) = (primal0 + primalt, tangent0 + tangentt);
    let (primal2, tangent2) = (primal1 + 1.0, tangent1);
    (primal2, tangent2)
}


pub fn run() {
    let t = 5.0;
    println!("Symbolic: {}", df_sym(t));

    let h = 1e-6;
    println!("Numeric: {}", df_num(t, h));

    println!("Automatic: {}", f_ad((t, 1.0)).1);
}