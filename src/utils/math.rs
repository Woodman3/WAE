use std::ops::{Add, Mul, Sub};
#[macro_export]
macro_rules! add2d {
    () => {};
    // (($($a:expr),*),($($b:expr),*)) => {
    //    ($($a+$b),*)
    // };
    ($a:expr,$b:expr) => {
        ($a.0 + $b.0, $a.1 + $b.1)
    };
}
#[macro_export]
macro_rules! sub2d {
    () => {};
    ($a:expr,$b:expr) => {
        ($a.0 - $b.0, $a.1 - $b.1)
    };
}
#[macro_export]
macro_rules! mul2d {
    () => {};
    ($a:expr,$b:expr) => {
        ($a.0 * $b.0 + $a.1 * $b.1)
    };
}
///we define A is start_point,B is end_point,P is target_point in short
pub fn distance_from_segment_to_point(A: &(f64, f64), B: &(f64, f64), P: &(f64, f64)) -> f64 {
    let AB = sub2d!(B, A);
    let AP = sub2d!(P, A);
    let r = mul2d!(AB, AP) / mul2d!(AB, AB);
    if r >= 1.0 {
        let BP = sub2d!(P, B);
        mul2d!(BP, BP).sqrt()
    } else if r <= 0.0 {
        let AP = sub2d!(P, A);
        mul2d!(AP, AP).sqrt()
    } else {
        let AC = (r * AB.0, r * AB.1);
        mul2d!(AC, AC).sqrt()
    }
}

pub fn distance_p2p<T,T2>(A:&(T,T),B:&(T2,T2))->f64
where  T:Into<f64>+Copy,
       T2:Into<f64>+Copy
{
    let a=(A.0.into(),A.1.into());
    let b=(B.0.into(),B.1.into());
    let AB=sub2d!(b,a);
    let d=mul2d!(AB,AB);
    d.sqrt()
}
