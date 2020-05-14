pub fn references_test() {
    change_reference();

    local_borrow_reference_dangling();

    reference_dangling_due_to_move();
}

fn reference_dangling_due_to_move() {
    let v = vec![1, 3, 4, 5, 6, 2];
    let r = &v;
    let x = v;
    // cannot move out of v because it is borrow
    //r[0];

    // let mut x = 10;
    // let z = &x;
    // x = 1;
    // println!("{}",z);
}

fn uninitialized_reference_variable() {
    let r: &i32;
    // borrow of uninitialized variable
    //println!("{}",r);
}

fn local_borrow_reference_dangling() {
    let r: &i32;
    {
        let x = 10;
        r = &x;
        println!("{}", *r);
    } // x goes out of scope

    // using reference here will result in error
    //assert_eq!(*r,10);
}

fn change_reference() {
    let x = 10;
    let y = 20;

    let mut a = &x;
    let mut b = &y;

    let v = *a;

    println!("Dereferenced value: {}", v);

    if true {
        b = a;
        println!("after re-assign: {}", b);
    }
    return;
}
