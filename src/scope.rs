use std::ops::Neg;

#[derive(Clone)]
pub enum Scope{
    Rect(Vec<((f64,f64),(f64,f64))>),//from left-up to right-down,the default toward is East
    Circle((f64,f64,f64))//first and second is coordinate of circle ,third is radius
}
pub enum Toward{
    North,
    South,
    East,
    West
}
impl Scope{
    fn apply_toward(&self,to:Toward)->Scope{
        let mut s =self.clone();
        match &mut s{
            Scope::Rect(r) => {
                match to {
                    Toward::North => {
                        // for (x,y) in r.iter_mut(){
                        //     *x=-*x;
                        //     *y=-*y;
                        //     std::mem::swap(x,y);
                        // }
                        s
                    }
                    Toward::South => {s}
                    Toward::East => {s}
                    Toward::West => {
                        for ((x1,y1),(x2,y2)) in r.iter_mut(){

                        }
                        s
                    }
                }

            }
            Scope::Circle(_) => {s}
        }
    }
}