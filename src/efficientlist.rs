type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    element: T,
    next: Link<T>,
}

pub struct EfficientList<T> {
    head: Link<T>,
}

impl<T> Drop for EfficientList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> EfficientList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_using_option_enum(&mut self, element: T) {
        let new_node = Box::new(Node {
            element: element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop_using_option_enum(&mut self) -> Option<T> {
        let x = self.head.take();

        match x {
            Some(y) => {
                self.head = y.next;
                Some(y.element)
            }
            None => None,
        }
    }

    pub fn pop_using_option_enum2(&mut self) -> Option<T> {
        let x = self.head.take();
        x.map(|node| {
            self.head = node.next;
            node.element
        })

        // match self.head.take() {
        //     Some(x) => {
        //         self.head = x.next;
        //         Some(x.element)}
        //         ,
        //     None => None,
        // }
    }

    pub fn peek_using_option_as_ref(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(boxed_node) => Some(&boxed_node.element),
            None => None,
        }
    }

    pub fn push(&mut self, element: T) {
        // capture previous head
        // set new node next to previously captured head
        // set head to new node
        let previous_head = std::mem::replace(&mut self.head, None);

        let new_node = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));

        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        // capture head
        // new head = captured head next

        let previous_head = std::mem::replace(&mut self.head, None);

        match previous_head {
            Some(x) => {
                self.head = x.next;
                Some(x.element)
            }
            None => None,
        }
    }
}

pub fn efficient_list_test() {
    println!("Efficient list test");
    let mut list = EfficientList::new();
    list.push(10);
    list.push(20);

    assert_eq!(list.peek(), Some(&20));

    assert_eq!(list.pop(), Some(20));
    assert_eq!(list.pop(), Some(10));
    assert_eq!(list.pop(), None);
    assert_eq!(list.pop(), None);

    println!("End of efficient list test")
}

pub fn efficient_list_using_option_test() {
    println!("Efficient list: push/pop using option enum test");
    let mut list = EfficientList::new();
    list.push_using_option_enum(10);
    list.push_using_option_enum(20);
    assert_eq!(list.peek_using_option_as_ref(), Some(&20));

    drop(list);

    // assert_eq!(list.pop_using_option_enum2(), Some(20));
    // assert_eq!(list.pop_using_option_enum2(), Some(10));
    // assert_eq!(list.pop_using_option_enum2(), None);
    // assert_eq!(list.pop_using_option_enum2(), None);

    println!("End of Efficient list: push/pop using option enum test")
}

pub fn efficient_list_into_iter_test() {
    pub struct IntoIter<T>(EfficientList<T>);

    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    impl<T> EfficientList<T> {
        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }
    }

    let mut list = EfficientList::new();
    list.push(10);
    list.push(20);
    list.push(30);

    let mut into_iter_list = list.into_iter();

    while let Some(item) = into_iter_list.next() {
        println!("Popped value: {}", item);
    }
    assert_eq!(into_iter_list.next(), None);
}

pub fn efficient_list_iter_test() {
    println!("Efficient list iter test..");

    let mut l = EfficientList::new();
    l.push(10);
    l.push(20);
    l.push(30);

    let mut it = l.iter();

    while let Some(x) = it.next() {
        println!("Iterator Item: {}", x);
    }

    while let Some(x) = l.pop() {
        println!("Popped Item: {}", x);
    }

    println!("End of efficient list iter test");
}

// next
// next = next.next

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> EfficientList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // let y = self.head.as_ref();
        // let x = y.map(|node| &**node);

        // IntoIter {
        //     next : x
        // }

        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // below code shows how it works step by step
        // let a = self.next;
        // let b = a;
        // let c = a;

        // a.map(|x| {
        //     let a = self.next;
        //     let d = x.next.as_ref();
        //     let z = d.map(|sdf|
        //         {
        //             &**sdf
        //         }
        //         );
        //     self.next = z;
        //     &x.element
        // } );

        //next = next.next;
        self.next.map(|node| {
            // let x = node.next.as_ref();
            // let y = x.map(|n| &**n);
            // self.next = y;

            // pub fn map<U, F>(self, f: F) -> Option<U>
            // turbofish syntax to help compiler for deref coercion
            // map only caress about return type &Node<T> but not other parameter so _
            // compiler applies deref coercion to &node
            //self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);

            self.next = node.next.as_ref().map(|n| &**n);
            &node.element
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> EfficientList<T> {
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        // let x = self.head.as_mut();
        // let z = x.map(|node| {
        //     &mut **node
        // });
        // IterMut {
        //     next: z
        // }

        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.next.take();
        let y = x.map(|node| {
            // let a = node.next.as_mut();
            // let b = a.map(|n| &mut **n);
            // self.next = b;

            self.next = node.next.as_mut().map(|n| &mut **n);
            &mut node.element
        });
        y
    }
}

pub fn efficient_list_iter_mut_test() {
    println!("Efficient list iter_mut test..");

    let mut l = EfficientList::new();
    l.push(10);
    l.push(20);
    l.push(30);

    let mut it = l.iter_mut();

    while let Some(x) = it.next() {
        println!("Iterator Item: {}", x);
        *x *= 2;
    }

    while let Some(x) = l.pop() {
        println!("Popped Item: {}", x);
    }

    println!("End of efficient list iter_mut test");
}
