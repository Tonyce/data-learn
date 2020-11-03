use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;
use std::rc::Weak;

type ReadNode<T> = Rc<RefCell<Node<T>>>;
type DoubleLink<T> = Option<ReadNode<T>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: DoubleLink<T>,
    prev: WeakLink<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> ReadNode<T> {
        Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: i32,
    head: DoubleLink<T>,
    tail: DoubleLink<T>,
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
            Some(old) => {
                node.borrow_mut().prev = Some(Rc::downgrade(&old));
                old.borrow_mut().next = Some(node.clone());
            }
            None => {
                self.head = Some(node.clone());
            }
        }
        self.length += 1;
        self.tail = Some(node);
    }

    fn prepend(&mut self, data: T) {
        let node = Node::new(data);
        match self.head.take() {
            Some(old) => {
                node.borrow_mut().next = Some(old.clone());
                old.borrow_mut().prev = Some(Rc::downgrade(&node));
            }
            None => {
                self.tail = Some(node.clone());
            }
        }
        self.length += 1;
        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take(); // 把尾也置为 None，即使是 None
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
struct DoubleLinkIterator<T> {
    current: DoubleLink<T>,
}

impl<T> DoubleLinkIterator<T> {
    fn new(double_link: DoubleLink<T>) -> Self {
        DoubleLinkIterator {
            current: double_link,
        }
    }
}

impl<T: std::clone::Clone> Iterator for DoubleLinkIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.data.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

trait BackIterator {
    type Item;
    fn prev(&mut self) -> Option<Self::Item>;
}

impl<T: std::clone::Clone> BackIterator for DoubleLinkIterator<T> {
    type Item = T;
    fn prev(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.data.clone());
                match &current.prev {
                    Some(inner) => inner.upgrade(),
                    None => None,
                }
            }
            None => None,
        };
        result
    }
}

fn main() {
    println!("Hello, world!");
    let mut t: LinkedList<i32> = LinkedList::new();
    t.append(2);
    t.append(3);
    println!("{:?}", t.length);
    t.prepend(1);
    t.pop();
    println!("{:?}", t);

    let mut iterator = DoubleLinkIterator::new(t.head.clone());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.prev());

    println!("{:?}", iterator.next());
}
