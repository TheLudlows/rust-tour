use std::fs::read;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: u32,
}

pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(v: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {
            val: v,
            next,
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> SimpleLinkedList<T> {
        SimpleLinkedList {
            head: None,
            len: 0,
        }
    }
    pub fn add(&mut self, e: T) {
        self.head = Some(Box::new(Node::new(e, self.head.take())));
        self.len += 1;
    }
    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn rem(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let mut node = self.head.take().unwrap();
        self.head = node.next.take();
        Some(node.val)
    }
}

#[test]
fn test_box() {
    let mut b = Some(Box::new(10));
    let c = b.take();
    let d = c.unwrap();
    assert_eq!(b, None);
    //assert_eq!(c,Some(Box::new(10)));
    assert_eq!(d, Box::new(10));
}

#[test]
fn test_list() {
    let mut list = SimpleLinkedList::new();
    list.add(10);
    list.add(11);
    list.add(12);
    list.add(13);
    list.add(14);

    assert_eq!(Some(14), list.rem());
    assert_eq!(Some(13), list.rem());
    assert_eq!(Some(12), list.rem());
    assert_eq!(Some(11), list.rem());
    assert_eq!(Some(10), list.rem());
    assert_eq!(None, list.rem());

    let node = Node::new(1, Some(Box::new(Node::new(1, None))));
    println!(".")
}