pub fn closure_test() {
    println!("Closure test..");
    closure_test_array();
    closure_test_on_vector();
    println!("Closure test end..")
}

fn closure_test_array() {
    let mut arrays = [1, 2, 3, -2];

    println!("Before: {:?}", arrays);
    for i in arrays.iter_mut() {
        *i = *i + 1;
    }

    for i in arrays.iter().map(|x| x * x).filter(|&x| x < 25) {
        println!("{}", i)
    }

    println!("After: {:?}", arrays);
}

fn closure_test_on_vector() {
    vector_iter_and_print();
    vector_borrow_and_print();
    vector_mut_borrow_and_print();
    vector_move_and_print();

    vector_into_iterator_and_print();

    vector_iter_map();

    fn vector_iter_map() {
        println!("Vector Iterator test");
        let numbers = vec![1, 3, 4, 5];
        let iter = numbers.iter();

        let x: Vec<(usize, i32)> = iter.map(|number| *number).enumerate().collect();

        let z: Vec<&i32> = numbers.iter().filter(|x| **x > 1).map(|y| y).collect();

        let z: Vec<i32> = numbers.iter().filter(|x| **x > 1).map(|y| *y * 2).collect();

        for i in &z {
            print!("{} ", i);
        }
        println!();

        let y = numbers.into_iter();
        let z: Vec<i32> = y.map(|x| x).collect();
    }

    fn vector_iter_and_print() {
        let numbers = vec![1, 2];

        // using iterator
        let mut iter = numbers.iter();

        let value1 = iter.next();
        assert_eq!(value1, Some(&1));
        let value2 = iter.next();
        assert_eq!(value2, Some(&2));
        let value3 = iter.next();
        assert_eq!(value3, None);

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        // use while let pattern
        let mut iter = numbers.iter();
        while let Some(x) = iter.next() {
            print!("{} ", x);
        }
        println!();

        // using for loop
        for x in numbers.iter() {
            print!("{} ", *x);
        }
        println!();
    }

    fn vector_borrow_and_print() {
        let numbers = vec![1, 2, 3, 4, 6, 7, 8, 9, 10];
        // shared ref
        for x in &numbers {
            print!("{} ", *x);
        }
        println!();
    }

    fn vector_mut_borrow_and_print() {
        println!("Vector mut borrow and print");

        let mut numbers = vec![1, 3, 5];

        println!("After squaring...");
        for n in &mut numbers {
            *n *= *n;
            print!("{} ", *n);
        }
        println!();

        println!("After doubling...");

        let mut mut_iter = (&mut numbers).into_iter();
        while let Some(x) = mut_iter.next() {
            *x *= 2;
            print!("{} ", x);
        }
        println!();

        let mut iter = numbers.into_iter();
        while let Some(x) = iter.next() {
            print!("{} ", x);
        }
        println!();
    }

    fn vector_move_and_print() {
        let numbers = vec![1, 2, 3, 4, 6, 7, 8, 9, 10];
        // move
        for x in numbers {
            print!("{} ", x);
        }
        println!();
    }

    fn vector_into_iterator_and_print() {
        let numbers = vec![1, 2];

        println!("Using IntoIter trait into_iter");
        let mut moved_value = numbers.into_iter();

        while let Some(x) = moved_value.next() {
            print!("{} ", x);
        }
        println!();

        use_borrow_and_then_call_into_iter();

        fn use_borrow_and_then_call_into_iter() {
            let numbers = vec![1, 3];

            let mut x = (&numbers).into_iter();
            while let Some(z) = x.next() {
                print!("{} ", z);
            }
            println!();

            println!("{}", numbers.len());

            let mut x = (&numbers).into_iter();
            let value1 = x.next();
            assert_eq!(value1, Some(&1));
            let value2 = x.next();
            assert_eq!(value2, Some(&3));
            let value3 = x.next();
            assert_eq!(value3, None);
        }
    }
}
