use std::mem::swap;

#[derive(Clone,Debug)]
pub enum Scope{
    Rect(Vec<((i32,i32),(i32,i32))>),//from left-up to right-down,the default toward is East
    Circle((f64,f64,f64))//first and second is coordinate of circle ,third is radius
}
#[derive(Debug,Clone)]
pub enum Toward{
    North,
    South,
    East,
    West
}
impl Scope{
    pub fn apply_toward(&self,to:&Toward)->Scope{
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
    /// if loc is operator,it is row and col
    /// if loc is enemy,it is x and y
    /// this two is invert
    pub fn apply_loc<T:>(&mut self,loc:((T,T)),width:u32,height:u32)
    where T:Into<i32>+Into<f64>
    {
        match self {
            Scope::Rect(v) => {
                let (row,col):(i32,i32)=(loc.0.into(),loc.1.into());
                for (ul,dr) in v{
                    ul.0+=row;
                    ul.1+=col;
                    if ul.0<0{ul.0=0}
                    if ul.1<0{ul.1=0}
                    dr.0+=row;
                    dr.1+=col;
                    if dr.0 >(height-1) as i32{dr.0=(height-1) as i32}
                    if dr.1 >(width-1) as i32{dr.1=(width-1) as i32}
                }
            }
            Scope::Circle(c) => {
                let (x,y):(f64,f64)=(loc.0.into(),loc.1.into());
                c.0+=x;
                c.1+=y;
            }
        }
    }
}
