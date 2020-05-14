pub fn list_test() {
    not_efficient_list_representation();

    another_enum_representation_for_list();

    efficient_list_representation();

    list_operation_test();

    fn list_operation_test() {
        let mut empty_list = ManualList::new();

        empty_list.push(100);
        empty_list.push(300);

        assert_eq!(empty_list.pop(), Some(300));
        assert_eq!(empty_list.pop(), Some(100));
        assert_eq!(empty_list.pop(), None);

        println!("End of push pop test");
    }

    fn not_efficient_list_representation() {
        #[derive(Debug)]
        struct Point {
            x: f64,
            y: f64,
        }

        // probable null pointer optimization
        #[derive(Debug)]
        enum RustEnumList {
            Empty,
            Element(u32, Box<RustEnumList>),
        }
        let list = RustEnumList::Empty;

        println!("Size: {}", std::mem::size_of_val(&list));

        // [] = Stack
        // () = Heap
        // [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)

        // non-uniform node layout because of mixed allocation(some stack, some heap)
        // Junk for RustEnumList::Empty Representation
        // No efficient split or merge operations(requires move from heap to stack and vice versa)

        let list2 = RustEnumList::Element(
            1,
            Box::new(RustEnumList::Element(2, Box::new(RustEnumList::Empty))),
        );
        println!("Value: {:?},Size: {}", list2, std::mem::size_of_val(&list2));
    }

    fn another_enum_representation_for_list() {
        println!("Another enum representation of list");
        #[derive(Debug)]
        pub enum List {
            Empty,
            ElemThenEmpty(i32),
            ElemThenNotEmpty(i32, Box<List>),
        }

        // logically invalid state
        //let l = List::ElemThenNotEmpty(0, Box::new(List::Empty));
        // but no empty allocation with junk value ( if ElemThenEmpty is used)

        // and still non uniform node layout(some stack , some heap)
        // and no null pointer optimization so more space wastage
        let l = List::ElemThenNotEmpty(
            1,
            Box::new(List::ElemThenNotEmpty(1, Box::new(List::ElemThenEmpty(20)))),
        );

        use std::mem::size_of_val;
        //println!("Size {:?} -> {}",List::Empty,size_of_val(&List::Empty));

        let after_empty = List::ElemThenEmpty(20);
        println!("Size {:?} -> {}", &after_empty, size_of_val(&after_empty));

        let after_notempty = List::ElemThenNotEmpty(30, Box::new(List::ElemThenEmpty(5)));
        println!(
            "Size {:?} -> {}",
            after_notempty,
            size_of_val(&after_notempty)
        );

        println!("Size: {}", size_of_val(&l));
    }

    pub fn efficient_list_representation() {
        println!("Efficient list representation..");

        use std::mem::size_of;

        println!("Size of List struct: {}", size_of::<ManualList>());
        println!("Size of link enum : {}", size_of::<Link>());
        println!("Size of node struct : {}", size_of::<Node>());

        // let l = List::Empty;
        // let non_empty = List::Node(Box::new(Link {
        //     element: 20,
        //     next: List::Empty,
        // }));

        let list = ManualList::new();
        let l = ManualList {
            head: Link::Element(Box::new(Node {
                element: 20,
                next: Link::Empty,
            })),
        };
    }
}

// [] = Stack
// () = Heap
//[ptr] -> (Elem A, ptr) -> (Elem B, *null*)
// null pointer optimization possible

// List a = Empty | Cons of a * List B

// zero cost abstraction since only contains single element
pub struct ManualList {
    head: Link,
}

impl Drop for ManualList {
    fn drop(&mut self) {
        // this prevents unbounded recursion
        // tail recursion not possible due to Box
        let mut previous_head = std::mem::replace(&mut self.head, Link::Empty);

        while let Link::Element(mut boxed_node) = previous_head {
            previous_head = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }

        // let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // // `while let` == "do this thing until this pattern doesn't match"
        // while let Link::More(mut boxed_node) = cur_link {
        //     cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        //     // boxed_node goes out of scope and gets dropped here;
        //     // but its Node's `next` field has been set to Link::Empty
        //     // so no unbounded recursion occurs.
        // }
    }
}

enum Link {
    Empty,
    Element(Box<Node>),
}

struct Node {
    element: u32,
    next: Link,
}

impl ManualList {
    pub fn new() -> Self {
        let empty = Link::Empty;
        Self { head: empty }
    }

    pub fn push(&mut self, element: u32) {
        let previous_head = std::mem::replace(&mut self.head, Link::Empty);

        let new_node = Node {
            element: element,
            next: previous_head,
        };

        self.head = Link::Element(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<u32> {
        let previous_head = std::mem::replace(&mut self.head, Link::Empty);

        match previous_head {
            Link::Empty => None,
            Link::Element(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }

        // match &self.head {
        //     Link::Empty => None,
        //     Link::Element(ref x) => {
        //         //let z = std::mem::replace(&mut self.head, x.next);
        //         self.head = x.next;
        //         Some(x.element)
        //     }
        // }
    }
}
