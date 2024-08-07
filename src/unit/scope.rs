use crate::utils::math::GridRect;
use serde::{Deserialize, Serialize};

///from left-up to right-down,the default toward is East
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Scope(pub Vec<GridRect>);

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub(crate) enum Toward {
    #[serde(alias = "Up")]
    North,
    #[serde(alias = "Down")]
    South,
    #[default]
    #[serde(alias = "Right", alias = "None")]
    East,
    #[serde(alias = "Left")]
    West,
}
impl Scope {
    pub(crate) fn apply_toward(&mut self, to: &Toward) {
        use std::mem::swap;
        match to {
            Toward::North => {
                // reflect by y=x and then change two point
                for r in self.0.iter_mut() {
                    swap(&mut r.ul.row, &mut r.ul.col);
                    swap(&mut r.dr.row, &mut r.dr.col);
                    swap(&mut r.ul.row, &mut r.dr.row);
                    swap(&mut r.ul.col, &mut r.dr.col);
                }
            }
            Toward::South => {
                // reflect by y=-x
                for r in self.0.iter_mut() {
                    r.ul.row = -r.ul.row;
                    r.ul.col = -r.ul.col;
                    r.dr.row = -r.dr.row;
                    r.dr.col = -r.dr.col;
                    swap(&mut r.ul.row, &mut r.ul.col);
                    swap(&mut r.dr.row, &mut r.dr.col);
                    swap(&mut r.ul.row, &mut r.dr.row);
                    swap(&mut r.ul.col, &mut r.dr.col);
                }
            }
            Toward::East => {}
            Toward::West => {
                // reflect by origin and then change two point
                for r in self.0.iter_mut() {
                    r.ul.row = -r.ul.row;
                    r.ul.col = -r.ul.col;
                    r.dr.row = -r.dr.row;
                    r.dr.col = -r.dr.col;
                    swap(&mut r.ul.row, &mut r.dr.row);
                    swap(&mut r.ul.col, &mut r.dr.col);
                }
            }
        }
    }
    /// if loc is operator,it is row and col
    /// if loc is enemy,it is x and y
    /// this two is invert
    pub(crate) fn apply_loc<T>(&mut self, loc: (T, T), width: u32, height: u32)
    where
        T: Into<i64>,
    {
        let (row, col): (i64, i64) = (loc.0.into(), loc.1.into());
        for r in self.0.iter_mut() {
            r.ul.row += row;
            r.ul.col += col;
            if r.ul.row < 0 {
                r.ul.row = 0
            }
            if r.ul.col < 0 {
                r.ul.col = 0
            }
            r.dr.row += row;
            r.dr.col += col;
            if r.dr.row > (height - 1) as i64 {
                r.dr.row = (height - 1) as i64
            }
            if r.dr.col > (width - 1) as i64 {
                r.dr.col = (width - 1) as i64
            }
        }
    }
}
