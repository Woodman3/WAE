
struct Frame{
    set:Vec<i32>,
    buffer : Vec< *const i32>,
}

impl Frame{
    fn new()->Self{
        Frame{
            set:vec![1,2,3],
            buffer:Vec::new(),
        }
    }

    fn fun(&mut self){
        self.buffer.push(&self.set[1] as *const i32);
    }
}

#[test] 
fn fun(){
    let mut f = Frame::new();
    f.fun();
    let t = f.buffer[0];
    unsafe{
        assert_eq!(*t,2);
    }
}
