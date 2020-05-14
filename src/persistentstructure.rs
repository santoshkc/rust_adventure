use std::rc::Rc;

pub struct PersistentStack<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> PersistentStack<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&self, element: T) -> PersistentStack<T> {
        // Option<Rc<Node<T>>>
        // // list + element => list
        //let rc = self.head.as_ref().map(|r| r.clone());

        let rc = self.head.clone();

        let node = Node {
            element: element,
            next: rc,
        };

        PersistentStack {
            head: Some(Rc::new(node)),
        }
    }

    pub fn head(&self) -> Option<&T> {
        let head = self.head.as_ref().and_then(|x| Some(&x.element));
        head
    }

    pub fn tail(&self) -> PersistentStack<T> {
        // let tail_clone = self.head.as_ref().map(|node|{
        //     node.next.clone()
        // });

        // let z = match tail_clone {
        //     Some(x) => x,
        //     None => None,
        // };

        // PersistentStack {
        //     head: z
        // }

        PersistentStack {
            head: self.head.as_ref().and_then(|x| x.next.clone()),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> PersistentStack<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.element
        })
    }
}

pub fn test_persistent_stack_iter() {
    println!("Persistent Stack Iterator");
    let list = PersistentStack::new()
        .append(10)
        .append(20)
        .append(30)
        .append(40)
        .append(50);

    let mut iter = list.iter();
    while let Some(x) = iter.next() {
        println!("Items: {}", x);
    }
    println!("Persistent Stack Iterator End");
}

pub fn test_persistent_stack() {
    println!("Persistent Stack Test");
    let stack: PersistentStack<i32> = PersistentStack::new();

    let new_stack = stack.append(20);

    let second_stack_copy = new_stack.append(30);

    let head1 = second_stack_copy.head();
    println!("Head 1 : {:?}", head1);

    let tail1 = second_stack_copy.tail();

    let head2 = tail1.head();
    println!("Head 2 : {:?}", head2);
    let tail2 = tail1.tail();

    println!("End of Persistent stack test");
}
