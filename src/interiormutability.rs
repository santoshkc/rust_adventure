use std::cell::{Cell, RefCell};
use std::{rc::Rc, thread};

pub fn interior_mutability_test() {
    let mut x = &10;

    println!("{:p}", x);

    x = &20;

    let result = x == &20;

    let ptr_equal = std::ptr::eq(x, &10);

    println!("Result: {}, {},{:p} , {:p}", result, ptr_equal, x, &20);

    return;

    reference_counting_test();

    rc_immutable_referent();

    use_of_cell_for_mutation_in_struct();
    rc_mutating_referent_using_RefCell();
}

#[derive(Debug)]
struct Person {
    name: RefCell<String>,
    age: Cell<u8>,
    salary: u32,
}

impl Person {
    fn set_age(&self, age: u8) {
        self.age.set(age);
    }

    fn get_age(&self) -> u8 {
        self.age.get()
    }
}

fn use_of_cell_for_mutation_in_struct() {
    let person = Person {
        name: RefCell::new(String::from("Santosh")),
        age: Cell::new(31),
        salary: 2000,
    };

    println!("Person: {:?}", person);

    let age = person.get_age();
    println!("Age Now:  {}", age);

    person.set_age(40);
    let age = person.get_age();
    println!("After mutation, Age:  {}", age);

    // use RefCell for mutable borrow
    println!("Person: {:?}", person);

    let mut mut_name = person.name.borrow_mut();
    mut_name.push_str(" K.C.");
    drop(mut_name);

    println!("Valu after mut borrow: Person: {:?}", person);

    // let mut cell_name = Cell::new(person.name);
    // let name = cell_name.get_mut();
    // *name.push_str("!!!");
}

fn rc_mutating_referent_using_RefCell() {
    let msg = "**Hello World";
    println!("Initial msg: {}", msg);

    let rc_refcell_data = Rc::new(RefCell::new(msg.to_string()));

    println!("{:?}", rc_refcell_data);
    println!("**Reference Count: {}", Rc::strong_count(&rc_refcell_data));

    {
        let clone = rc_refcell_data.clone();
        println!("{:?}", clone);

        let string_data = clone.borrow();
        println!("String: {:?}", string_data);

        // drop shared borrow here so that we could borrow mutably below.
        // failing to do so will cause panic
        drop(string_data);

        let mut mutable_string_data = clone.borrow_mut();
        mutable_string_data.push_str("!!!");
        println!("Mutated Msg: {}", mutable_string_data);

        //// uncommenting borrowing code below causes panic because it is already borrowed mutably
        // let borrowed_string = clone.borrow();
        // println!("Borrowed string length: {}",borrowed_string.len());

        println!("**Reference Count: {}", Rc::strong_count(&clone));
    }
    println!("{:?}", rc_refcell_data);
    println!("**Reference Count: {}", Rc::strong_count(&rc_refcell_data));
}

fn rc_immutable_referent() {
    let rc1 = Rc::new("**Hello World".to_owned());
    println!(
        "Value: {}, Length: {}, Capacity: {}, Address: {:p}",
        rc1,
        rc1.len(),
        rc1.capacity(),
        rc1
    );

    // error if tried for mutable borrow
    //rc1.push_str("!!");

    // could not use Rc inside thread
    // let rc2 = rc1.clone();
    // thread::spawn(|| {
    //     let length = rc2.len();

    // });

    // thread::spawn(move || {
    //     let length = rc1.len();

    // });
}

pub fn reference_counting_test() {
    let rc1 = Rc::new("Hello World!!");

    let count = Rc::strong_count(&rc1);
    println!("Counter: {}", count);

    println!("Rc Address: {:p}, Data address: {:p}", &rc1, &(*rc1));

    let data_reference = &(*rc1);
    println!(
        "Value: {:p}, pointer: {:p}",
        data_reference, &data_reference
    );

    let rc1_clone = rc1.clone();
    println!(
        "Rc Address: {:p}, Data address: {:p}",
        &rc1_clone,
        &(*rc1_clone)
    );

    let rc2_clone = rc1.clone();
    println!(
        "Rc Address: {:p}, Data address: {:p}",
        &rc2_clone,
        &(*rc2_clone)
    );

    let count = Rc::strong_count(&rc1);

    println!(
        "Counter: {count}, Value: {v}, Addresses:  {r:p} {r1:p}, {r2:p}",
        v = rc1,
        r = rc1,
        count = count,
        r1 = rc1_clone,
        r2 = rc2_clone
    );

    drop(rc1_clone);
    drop(rc2_clone);

    let count = Rc::strong_count(&rc1);
    println!("Counter: {}", count);
}
