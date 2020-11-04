use std::cell::RefCell;
use std::rc::Rc;

type RealNode<T> = Rc<RefCell<Node<T>>>;
type SingleLink<T> = Option<RealNode<T>>;

#[derive(Debug)]
struct LinkedList<T> {
    length: i32,
    head: SingleLink<T>,
    tail: SingleLink<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }
    fn append(&mut self, data: T) {
        let node = Node::new(data);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(node.clone()),
            None => self.head = Some(node.clone())
        }
        self.length += 1;
        self.tail = Some(node.clone());
    }

    fn prepend(&mut self, data: T) {
        let node = Node::new(data);
        match self.head.take() {
            Some(old) => node.borrow_mut().next = Some(old),
            None => {
                self.tail = Some(node.clone());
            }
        }
        self.length += 1;
        self.head = Some(node.clone());
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;

            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .data
        })
    }
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: SingleLink<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> RealNode<T> {
        Rc::new(RefCell::new( Node {
            data,
            next: None,
        }))
    }
}

fn main() {
    let mut t: LinkedList<i32> = LinkedList::new();
    t.append(3);
    t.prepend(2);
    println!("{:?}", t);
    t.prepend(1);
    println!("{:?}", t);
    t.pop();
    println!("{:?}", t);
}
