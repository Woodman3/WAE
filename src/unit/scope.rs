use std::mem::swap;

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
                use std::mem::swap;
                match to {
                    Toward::North => {
                        /// reflect by y=x and then change two point
                        for ((x1,y1),(x2,y2)) in r.iter_mut(){
                            swap(x1,y1);
                            swap(x2,y2);
                            swap(x1,x2);
                            swap(y1,y2);
                        }
                        s
                    }
                    Toward::South => {
                        /// reflect by y=-x
                        for ((x1,y1),(x2,y2)) in r.iter_mut(){
                            *x1=-*x1;
                            *y1=-*y1;
                            *x2=-*x2;
                            *y2=-*y2;
                            swap(x1,y1);
                            swap(x2,y2);
                            swap(x1,x2);
                            swap(y1,y2);
                        }
                        s
                    }
                    Toward::East => {s}
                    Toward::West => {
                        /// reflect by origin and then change two point
                        for ((x1,y1),(x2,y2)) in r.iter_mut(){
                            *x1=-*x1;
                            *y1=-*y1;
                            *x2=-*x2;
                            *y2=-*y2;
                            swap(x1,x2);
                            swap(y1,y2);
                        }
                        s
                    }
                }

            }
            Scope::Circle(_) => {s}
        }
    }
}