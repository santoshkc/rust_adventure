// double linked list
// node contains next and previous pointer
// tracks first and last element
// for fast insertion and removal at both ends

use std::cell::*;
use std::rc::Rc;

// data type
type T = u32;

type Link = Option<Rc<RefCell<Node<T>>>>;

struct DoubleLinkedList {
    first: Link,
    last: Link,
}

struct Node<T> {
    element: T,
    previous: Link,
    next: Link,
}

impl DoubleLinkedList {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    pub fn peek_at_front(&self) -> Option<Ref<T>> {
        let front = self.first.as_ref().map(|node| {
            let b = node.borrow();

            let r = Ref::map(b, |n| &n.element);
            r
        });
       
        front
    }

    pub fn peek_mut_at_front(&mut self) -> Option<RefMut<T>> {
        let front = self.first.as_mut().map(|n|
        {
            let x = n.borrow_mut();
            RefMut::map(x,|t| &mut t.element)
        });

        front
    }

    pub fn peek_at_back(&self) -> Option<Ref<T>> {

        let last = self.last.as_ref().map(|node|{

            Ref::map(node.borrow(), |n| &n.element)

        });
        last
    }

    pub fn peek_mut_at_back(&mut self) -> Option<RefMut<T>> {
        let last = self.last.as_mut().map(|node|{

            RefMut::map(node.borrow_mut(), |t| &mut t.element)
        });
        last
    }

    pub fn append_at_end(&mut self, element: T) {
        // new_node.next => null
        // new_node.previous -> last
        // last = new_node

        // Option<Rc<RefCell<Node<T>>>>

        let owned_last = self.last.take();

        if let Some(previous_last) = owned_last {
            // we have first node
            // modify this first node
            // create new node and make this new first node

            let node = Node {
                element: element,
                next: None,
                previous: Some(previous_last.clone()),
            };

            let rc = DoubleLinkedList::NodeToLink(node);
            let mut mut_previous = previous_last.borrow_mut();
            mut_previous.next = rc.clone();

            self.last = rc;
        } else {
            // no first and last node
            // so make this node both first and last node

            let node = Node {
                element: element,
                previous: None,
                next: None,
            };

            let link = DoubleLinkedList::NodeToLink(node);

            self.first = link.clone();
            self.last = link;
        }
    }

    pub fn NodeToLink(node: Node<T>) -> Link {
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn append_at_start(&mut self, element: T) {
        let owned_first = self.first.take();

        if let Some(previous_first) = owned_first {
            // we have first node
            // modify this first node
            // create new node and make this new first node

            let node = Node {
                element: element,
                next: Some(previous_first.clone()),
                previous: None,
            };

            let rc = DoubleLinkedList::NodeToLink(node);
            let mut mut_previous = previous_first.borrow_mut();
            mut_previous.previous = rc.clone();

            self.first = rc;
        } else {
            // no first and last node
            // so make this node both first and last node

            let node = Node {
                element: element,
                previous: None,
                next: None,
            };

            let link = DoubleLinkedList::NodeToLink(node);

            self.first = link.clone();
            self.last = link;
        }
    }

    pub fn remove_at_first(&mut self) -> Option<T> {
        if let Some(previous_first) = self.first.take() {

            // following code fails because of more than 1 strong reference count
            // println!("Strong ref: {}",Rc::strong_count(&previous_first));
            // let z = Rc::try_unwrap(previous_first);
            // let mut node = z.ok().unwrap().into_inner();

            // node.next.take();
            // node.previous.take();
            // let element = node.element;

            // return Some(element);


            let rc_clone = previous_first.clone();

            let mut mut_node = rc_clone.borrow_mut();

            let next = mut_node.next.clone();

            mut_node.previous = None;
            mut_node.next = None;

            let element = mut_node.element;

            self.first = next;

            // if first is null, make last null as well
            if let None = self.first {
                self.last = None;
            }

            Some(element)
        } else {
            None
        }
    }

    pub fn remove_at_last(&mut self) -> Option<T> {
        if let Some(previous_last) = self.last.take() {
            let rc_clone = previous_last.clone();

            let mut mut_node = rc_clone.borrow_mut();

            let previous = mut_node.previous.clone();

            mut_node.previous = None;
            mut_node.next = None;

            let element = mut_node.element;

            self.last = previous;

            // if first is null, make last null as well
            if let None = self.last {
                self.first = None;
            }

            Some(element)
        } else {
            None
        }
    }

    pub fn insert(&mut self, element: T) {}

    pub fn iterate_from_first_to_last(&self) {
        println!("Iterating from first to last");

        let mut x = self.first.clone();
        // let mut x = self.first.as_ref()
        //     .and_then(|n| Some(n.clone()));
        while let Some(y) = x {
            let z = y.borrow().next.clone();
            print!("{} ", y.borrow().element);
            x = z;
        }
        println!();
        println!("Iterating from first to last ended..");
    }

    pub fn iterate_from_last_to_first(&self) {
        println!("Iterating from last to first");

        let mut x = self.last.clone();
        // let mut x = self.last.as_ref()
        // .and_then(|n|{
        //     Some(n.clone())
        // });

        while let Some(y) = x {
            let z = y.borrow().previous.clone();

            print!("{} ", y.borrow().element);
            x = z;
        }
        println!();
        println!("Iterating from last to first ended..");
    }
}

pub fn double_linked_list_test() {
    println!("Double linked list test..");
    let mut dll = DoubleLinkedList::new();

    for i in 10..20 {
        dll.append_at_end(i);
    }

    for i in (0..10).rev() {
        dll.append_at_start(i)
    }

    dll.iterate_from_first_to_last();
    dll.iterate_from_last_to_first();

    let p = dll.peek_at_front().unwrap();
    println!("Peek Front: {}", p);
    drop(p);

    let p = dll.peek_at_back().unwrap();
    println!("Peek Back: {}", p);
    drop(p);

    let mut p = dll.peek_mut_at_front().unwrap();
    *p = 100;
    drop(p);

    let mut p = dll.peek_mut_at_back().unwrap();
    *p = 100;
    drop(p);


    let ascend = true;

    if ascend {
        let mut z = dll.remove_at_first();
        while let Some(x) = z {
            println!("{:?} ", x);
            z = dll.remove_at_first();
        }
    } else {
        let mut z = dll.remove_at_last();
        while let Some(x) = z {
            println!("{:?} ", x);
            z = dll.remove_at_last();
        }
    }

    let l = dll.remove_at_last();
    println!("Last: {:?}", l);

    let l = dll.remove_at_first();
    println!("First: {:?}", l);

    dll.iterate_from_first_to_last();
    dll.iterate_from_last_to_first();

    println!("Double linked list test end..")
}
