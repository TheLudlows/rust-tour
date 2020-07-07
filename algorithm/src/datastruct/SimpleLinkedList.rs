use std::cell::RefCell;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len : u32,
}
pub struct Node<T> {
    val:T,
    next:Option<Box<Node<T>>>
}
impl <T> Node<T> {
    pub fn new(v:T,next:Option<Box<Node<T>>>) -> Node<T> {
        Node{
            val:v,
            next
        }
    }
}
impl <T> SimpleLinkedList<T> {
    pub fn new()  -> SimpleLinkedList<T> {
        SimpleLinkedList{
            head:None,
            len : 0
        }
    }
    pub fn add(&mut self, e : T) {
        let node = Some(Box::new(Node::new(e,self.head.take())));
        self.head = node;
        self.len +=1
    }
    pub fn len(&self) -> u32{
        self.len
    }
}
#[test]
fn test_box(){
    let mut b = Some(Box::new(10));
    let c = b.take();
    let d = c.unwrap();
    assert_eq!(b,None);
    //assert_eq!(c,Some(Box::new(10)));
    assert_eq!(d,Box::new(10));
}
#[test]
fn test_list() {

}