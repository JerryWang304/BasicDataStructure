use std::fmt::Display;
struct List<T> where T: Display{
    head: Option<Box<Node<T>>>,
}
struct Node<T> where T: Display{
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> where T: Display{
    fn new(v: T) -> Self {
        Node {
            val: v,
            next: None,
        }
    }
}
impl<T> List<T> where T: Display{
    // empty or not
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // return the lenght of whole list
    fn len(&self) -> i32 {
        let mut start = &self.head;
        let mut count = 0;
        loop {
            match *start {
                Some(ref x) => {
                    count += 1;
                    start = &x.next;
                },
                None => break,
            }
        }
        return count;
    }

    // construct a new list
    fn new() -> Self {
        List {
            head: None,
        }
    }

    // insert a node after the head
    fn insert(&mut self, val: T) {
        let mut node = Node::new(val); // the new node 
        node.next = self.head.take();
        self.head = Some(Box::new(node));
    }
    fn show(&self) -> String {
        let mut ret: String = "".to_string();
        let mut start = &self.head;
        loop {
            match *start {
                None => break,
                Some(ref x) => {
                    ret.push_str(&*(x.val.to_string()));
                    start = &x.next;
                }
            }
        }
        return ret;
    }


}
fn main() {
    let mut my_list: List<i32> = List::new();
    for i in 0..10 {
        my_list.insert(i);
    }
    println!("length = {}",my_list.len());
    println!("the list is {}",my_list.show());
}
