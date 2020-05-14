mod person;

fn main() {
    // fizbuzz_printer(20);

    // palindrome_printer(100, 200);

    // closures::closure_test();
    //synchronization::synchronization_test();
    //rayontest::rayon_test();

    //rustchannels::channel_test();

    //rustchannels::channel_test_drop();

    //rustlist::list_test();

    doublelinkedstructure::double_linked_list_test();

    // persistentstructure::test_persistent_stack();
    // persistentstructure::test_persistent_stack_iter();

    // efficientlist::efficient_list_test();
    // efficientlist::efficient_list_using_option_test();
    // efficientlist::efficient_list_into_iter_test();
    // efficientlist::efficient_list_iter_test();
    // efficientlist::efficient_list_iter_mut_test();

    //interiormutability::interior_mutability_test();

    //reference_test::references_test();
}

mod doublelinkedstructure;

mod efficientlist;
mod persistentstructure;

mod closures;
mod interiormutability;
mod rayontest;
mod reference_test;
mod rustchannels;
mod rustlist;
mod synchronization;

static NAME: &'static str = "Santosh";
fn simple_struct_test() {
    let address = "Tinkune";
    let age = 31;
    let sex = Some(person::Sex::Male);
    {
        let p = person::Person::new(NAME, address, age, sex);

        match p.sex() {
            Some(gender) => match gender {
                person::Sex::Male => println!("Gender Identification: Male"),
                person::Sex::Female => println!("Gender Identification: Female"),
                person::Sex::Trans => println!("Gender Identification: Trans"),
            },
            None => println!("Gender : N/A"),
        }

        println!("Person Details: {}", p);
    }
}

fn palindrome_printer(start: u32, end: u32) {
    for i in start..=end {
        if is_palindrome(i) {
            println!("{} is Palindrome? {}", i, is_palindrome(i))
        }
    }

    fn is_palindrome(mut number: u32) -> bool {
        let clone = number;

        let mut final_result = 0;
        while number > 0 {
            let r = number % 10;
            final_result = final_result * 10 + r;
            number = number / 10;
        }

        clone == final_result
    }
}

fn fizbuzz_printer(n: u32) {
    for i in 1..=n {
        let result = match i {
            i if i % 15 == 0 => "fizbuzz",
            i if i % 3 == 0 => "fiz",
            i if i % 5 == 0 => "buzz",
            _ => continue,
        };
        println!("Result: {} -> {}", i, result);
    }
}
