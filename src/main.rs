//use std::rc::Rc;
use std::mem;
#[allow(dead_code)]
fn main() {
    let mut l = Link::new();
    l.insert(12);
    l.insert(120);
    l.insert(9);
    l.insert(-2);
    l.insert(-17);
    l.walk();
}

#[derive(Debug)]
struct Node {
    v: i32,
    next: Option<Box<Node>>,
}
impl Default for Node {
    fn default() -> Self {
        Node { v: 0, next: None }
    }
}

#[derive(Debug)]
struct Link {
    head: Box<Node>, // it would never to be None
                     //tail : Rc<Box<Node>>,
}

impl Node {
    fn new() -> Box<Self> {
        Box::new(Node { v: 0, next: None })
    }
    fn from(v: i32) -> Box<Self> {
        Box::new(Node { v, next: None })
    }
}
impl Link {
    fn new() -> Self {
        let head = Node::new();
        //		let tail = Rc::clone(&head);
        Link { head }
    }
    /** what i am struggle in is i am still thinking in a c style , i think i should make the new next pointer to the head node,
        but acutally , this is working in rust , it is wired, what i am actually doing is move head node to the new next , i am not use pointer anymore
        Problem here occure , when i move the head node to the new node , and i will move the new node to the head node postion , i need some thing that can do
        it will ,
        i can not move a head node to the new node
        cause i am use brrow
    **/

    fn insert(self: &mut Self, v: i32) {
        let mut new = Node::from(v);

        let old = mem::take(&mut self.head);

        new.next = Some(old);

        self.head = new;

        ()
    }
    fn walk(self: Self) {
        let mut cur = self.head;
        loop {
            if cur.next.is_none() {
                break;
            }
            print!(" {} -> ", cur.v);
            cur = cur.next.unwrap();
        }
        ()
    }
}
