use std::time::{Instant};
use std::ptr::NonNull;
use std::boxed::Box;
const NODE_NUMBER : usize = 30_00_00_00_0;
fn main(){
  let mut l = LinkList::new();
  let now = Instant::now();
  for i in 1..NODE_NUMBER{
      l.push_front_node(i as i32);
  }
  for _ in 1..NODE_NUMBER{
      l.pop_front_node();
  }
  println!("{} costs {} mills ", NODE_NUMBER,now.elapsed().as_millis());
}
type LINKNODE = Option<NonNull<Node>>;
struct LinkList{
    head : LINKNODE,
    tail : LINKNODE,
    len: usize,
}
#[derive(Debug)]
struct Node {
    elm:i32,
    next:LINKNODE,
}
impl Node{
    fn from(elm:i32)->Self{
        Node{elm,next:None}
    }
}
impl Drop for Node{
    fn drop(&mut self){
       // println!("I am dropped");
    }
}
impl Drop for LinkList{
    fn drop(&mut self){
        self.clear();
    }
}
impl LinkList{
    pub fn new()->Self{
        LinkList{head:None,tail:None,len:0}
    }
    pub fn push_front_node(&mut self,elm:i32){
        self.push_front_elm(elm);
        self.len += 1;
    }
     pub fn pop_front_node(&mut self)->Option<Node>{
        if self.len <=0{
            return None;
        }
        match self.drop_front_elm(){
		        None=>None,
		        Some(v)=>{self.len -=1;return Some(v);}
	      }
        
    }




    //pub fn pop_front_node(&mut self)->Option<Box<Node>>{
    //    if self.len <=0{
    //        return None;
    //    }
    //    match self.drop_front_elm(){
		//        None=>None,
		//        Some(v)=>{self.len -=1;return Some(v);}
	  //    }
    //    
    //}
    pub fn clear(&mut self){
        if self.len <=0{
            return ();
        }
        let mut ptr =self.head.unwrap().as_ptr(); // move NonNull 
        loop{
            unsafe{
                std::ptr::drop_in_place(self.head.unwrap().as_ptr());
                self.head = (*ptr).next;
            }
            self.len-= 1;
            if self.head.is_none(){
                break;
            }
            ptr = self.head.unwrap().as_ptr();
        }

    }
     fn drop_front_elm(&mut self)->Option<Node>{
        let old : NonNull<Node> = self.head.unwrap(); // it cannot move  it copy 
        let old_ptr = old.as_ptr(); // *mut Node
        unsafe{
            self.head = (*old_ptr).next;
            return Some(std::ptr::read(old_ptr));
        }
     }

    



//    fn drop_front_elm(&mut self)->Option<Box<Node>>{
//        let old : NonNull<Node> = self.head.unwrap(); // it cannot move  it copy 
//        let old_ptr = old.as_ptr(); // *mut Node
//        let node: Box<Node>;
//        unsafe{
//            node = Box::from_raw(old_ptr); // create a new one
//            self.head = (*old_ptr).next;
//            std::ptr::drop_in_place(old_ptr);
//        }
//        Some(node)
//
//    }
//    
    fn push_front_elm(&mut self,elm:i32){
        let node: Box<Node> = Box::new(Node::from(elm));

        let node_ptr: NonNull<Node> = NonNull::from(Box::leak(node));
        let ptr = node_ptr.as_ptr();
        unsafe{        
            (*ptr).next = self.head;
            (*ptr).elm = elm;
            if self.len == 0{
                self.tail = Some(node_ptr);
            }
            self.head = Some(node_ptr);
        }
    }
    
    pub fn as_vec(&self)->Option<Vec<i32>>{
	if self.len <= 0 {
		return None;
	}
        println!("len -> {}",self.len);
        let mut ptr:*mut Node = self.head.unwrap().as_ptr();
        let mut vec = Vec::with_capacity(self.len);
        loop{unsafe{
            vec.push((*ptr).elm);
            if (*ptr).next.is_none(){
                break;
            }
            ptr =(*ptr).next.unwrap().as_ptr();
        }
        }
        return Some(vec);
    }

}
