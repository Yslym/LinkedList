use std::rc::Rc;

struct Cg{
    id:Rc<Option<Box<i32>>>,
}
impl Cg{
    fn new(id:i32)->Self{
        Cg
        {
            id: Rc::new(Some(Box::new(id))),
        }
    }
    fn test_it(self:&mut Self){

       let i_get_id = Rc::clone(&self.id);
       ()
    }

}
fn main(){
    let cg = Cg::new(12);
    
}
