use serde::{Deserialize, Serialize};
use std::{fmt::{self, Display}, ops::{Add, Mul, Sub}};
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Debug, Copy, Default, Deserialize, Serialize, PartialEq)]
pub struct Grid {
    pub row: i64,
    pub col: i64,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(from = "[i64;4]")]
#[serde(into = "[i64;4]")]
pub struct GridRect {
    pub ul: Grid,
    pub dr: Grid,
}

impl From<(i64, i64)> for Grid {
    fn from(value: (i64, i64)) -> Self {
        Grid {
            row: value.0,
            col: value.1,
        }
    }
}

impl From<(Grid, Grid)> for GridRect {
    fn from(value: (Grid, Grid)) -> Self {
        GridRect {
            ul: value.0,
            dr: value.1,
        }
    }
}
impl From<(i64, i64, i64, i64)> for GridRect {
    fn from(value: (i64, i64, i64, i64)) -> Self {
        GridRect {
            ul: Grid::from((value.0, value.1)),
            dr: Grid::from((value.2, value.3)),
        }
    }
}

impl From<Point> for Grid {
    fn from(value: Point) -> Self {
        Grid {
            row: value.y as i64,
            col: value.x as i64,
        }
    }
}

impl Into<(i64, i64)> for Grid {
    fn into(self) -> (i64, i64) {
        (self.row, self.col)
    }
}
impl Into<(Grid, Grid)> for GridRect {
    fn into(self) -> (Grid, Grid) {
        (self.ul, self.dr)
    }
}
impl From<[i64; 4]> for GridRect {
    fn from(value: [i64; 4]) -> Self {
        (
            Grid::from((value[0], value[1])),
            Grid::from((value[2], value[3])),
        )
            .into()
    }
}
impl Into<[i64; 4]> for GridRect {
    fn into(self) -> [i64; 4] {
        [self.ul.row, self.ul.col, self.dr.row, self.dr.col]
    }
}
impl Into<(i64, i64, i64, i64)> for GridRect {
    fn into(self) -> (i64, i64, i64, i64) {
        (self.ul.row, self.ul.col, self.dr.row, self.dr.col)
    }
}

impl Into<(f64, f64)> for Point {
    fn into(self) -> (f64, f64) {
        (self.x.into(), self.y.into())
    }
}
impl Into<(f32, f32)> for Point {
    fn into(self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}
impl From<(f64, f64)> for Point {
    fn from(value: (f64, f64)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<(f32, f32)> for Point {
    fn from(value: (f32, f32)) -> Self {
        Point {
            x: value.0 as f64,
            y: value.1 as f64,
        }
    }
}
impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Point {
            x: value.0 as f64,
            y: value.1 as f64,
        }
    }
}
impl From<Grid> for Point {
    fn from(value: Grid) -> Self {
        Point {
            x: value.col as f64 + 0.5,
            y: value.row as f64 + 0.5,
        }
    }
}
impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: value.0 as f64,
            y: value.1 as f64,
        }
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

impl Mul for Point {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Add for Grid {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.row + rhs.row, self.col + rhs.col).into()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[macro_export]
macro_rules! add2d {
    () => {};
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
pub(crate) fn distance_from_segment_to_point(a: Point, b: Point, p: Point) -> f64 {
    // let AB = sub2d!(B, A);
    // let AP = sub2d!(P, A);
    // let r = mul2d!(AB, AP) / mul2d!(AB, AB);
    // if r >= 1.0 {
    //     let BP = sub2d!(P, B);
    //     mul2d!(BP, BP).sqrt()
    // } else if r <= 0.0 {
    //     let AP = sub2d!(P, A);
    //     mul2d!(AP, AP).sqrt()
    // } else {
    //     let AC = (r * AB.0, r * AB.1);
    //     mul2d!(AC, AC).sqrt() // this error!
    // }
    let ab = b - a;
    let ap = p - a;
    let r = (ab * ap) / (ab * ab);
    if r >= 1.0 {
        let bp = p - b;
        (bp * bp).sqrt()
    } else if r <= 0.0 {
        let ap = p - a;
        (ap * ap).sqrt()
    } else {
        let ac: Point = (r * ab.x, r * ab.y).into();
        let cp = ap - ac;
        (cp * cp).sqrt()
    }
}

// pub fn distance_p2p<T, T2>(a: &(T, T), b: &(T2, T2)) -> f64
// where
//     T: Into<f64> + Copy,
//     T2: Into<f64> + Copy,
// {
//     let a = (a.0.into(), a.1.into());
//     let b = (b.0.into(), b.1.into());
//     let ab = sub2d!(b, a);
//     let d = mul2d!(ab, ab);
//     d.sqrt()
// }

pub(crate) fn distance_p2p(a: &Point, b: &Point) -> f64 {
    let ab = *b - *a;
    (ab * ab).sqrt()
}

pub fn to_target(location: Point, target: Point, move_speed: f64) -> (Point, Point) {
    use crate::calculator::PERIOD;
    let direction = calculate_direction(target, location);
    let mut new = location.clone();
    new.x += move_speed * direction.x * PERIOD;
    new.y += move_speed * direction.y * PERIOD;
    (direction, new)
}
fn calculate_direction(target: Point, location: Point) -> Point {
    let delta = target - location;
    let theta = delta.y.atan2(delta.x);
    (theta.cos(), theta.sin()).into()
}
