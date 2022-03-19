pub trait CyRound<T> {
    fn cy_round(self) -> T;
}

impl<T> CyRound<T> for f64 where T: Round {
    fn cy_round(self) -> T {
        T::round(self)
    }
}

pub trait Round {
    fn round(x: f64) -> Self;
}

impl Round for i32 {
    fn round(x: f64) -> i32 {
        if x - (x as i32 as f64) >= 0.5 || is_double_approximately((x as i32 as f64) + 0.5, x) {
            (x as i32) + 1
        } else {
            x as i32
        }
    }
}

impl Round for i64 {
    fn round(x: f64) -> i64 {
        if x - (x as i64 as f64) >= 0.5 || is_double_approximately((x as i64 as f64) + 0.5, x) {
            (x as i64) + 1
        } else {
            x as i64
        }
    }
}

fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn is_double_approximately(a: f64, b: f64) -> bool {
    (b - a).abs()
        < partial_max(
            partial_max(a.abs(), b.abs()) * 1e-14,
            2.220_446_049_250_313e-16,
        )
}
