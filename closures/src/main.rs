fn main() {
    // Simple function

    let c = add_function(1, 1);
    println!("Result simple add_function: {}", c);

    // Closure equivalent

    let add_closure = |a: u32, b: u32| -> u32 { a + b };
    let d = add_closure(2, 2);
    println!("Result siple add_closure: {}", d);

    // Closures do not neet types

    let add_closure_no_types = |a, b| a + b;
    let e = add_closure_no_types(3, 3);
    println!("Result siple add_closure_no_types: {}", e);

    // I the code is one line, we can ommit braces {}

    let add_closure_no_types_no_braces = |a, b| a + b;
    let f = add_closure_no_types_no_braces(4, 4);
    println!("Result siple add_closure_no_types_no_braces: {}", f);

    // Super power of closures come from accessing variables from the scope outside of closure
    // this is what differs them from functions, except of that they may be very concise

    let printer_closure = || println! {"Results were: {c}, {d}, {e}, {f}"};
    printer_closure();

    // Here we create a function which returns a closure
    // the closure has captured `a` value which will be added to values
    // passed to closure

    /// impl Fn(i32) -> i32 means that this is a definition of a trait for closures
    /// which require parameters and output as well
    /// if the closure would not accept nor return nothing, it would be just impl Fn()
    fn create_adder(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }

    let add_5 = create_adder(5);
    println!("{}", add_5(4)); //prints 9
    println!("{}", add_5(20)); //prints 25

    // closures borrow immutably by default
    let i: String = "42".to_string(); // i is now a String, a non-copy type
    let capture_i = || println!("Inside closure: {i}");
    println!("Outside closure: {i}");
    capture_i();

    // But if the closure mutates the value from the outside, it borrows mutably
    // Below if we would change the order of `capture_animal()` and `println!` this would not compile
    let mut animal = "fox".to_string();
    let mut capture_animal = || {
        animal.push_str("es");
    };
    capture_animal(); // mutable borrow ends here
    println!("Outside closure: {animal}"); // Ok to use animal here

    // Closure may also move the variable, trying to access the `animal` after the code would fail
    let animal = "fox".to_string();
    let capture_animal = || {
        println!("Dropping {animal}");
        drop(animal);
    };
    capture_animal();

    // The `move` takes the ownership
    let i: String = "666".to_string();
    // added move before closure
    let capture_i = move || println!("Inside closure: {i}");
    // println!("Outside closure: {i}"); // <-- comment out this line to compile
    capture_i();

    // Example when the `move` is needed. Here since the string does not implement the
    // Copy trait, function `create_pluralizer` takes the ownership of the string.
    // Then since the function returns a closure, when the closure is called it want's
    // to access the string which was moved and owned by the `create_pluralizer` function
    // and dropped when the funtion was called.
    fn create_pluralizer(mut animal: String) -> impl FnMut() {
        move || {
            // <- `move` keyword added, without the `move` the closure wouldn't have the value
            animal.push_str("es");
            println!("Pluralized animal: {animal}");
        }
    }

    let my_string = "fox".to_string();
    let mut pluralize_fox = create_pluralizer(my_string);
    pluralize_fox();

    // Fn closure trait
    let i = 42;
    // Both capture_nothing and capture_i implement 'Fn'
    let capture_nothing = || println!("I capture nothing");
    let capture_i = || println!("I capture i immutably: {i}");
    call_closure_Fn_no_mutating_no_returnig(capture_nothing);
    call_closure_Fn_no_mutating_no_returnig(capture_i);

    //  FnMut closure trait
    let mut j = 0;
    let increment = || {j += 1; println!("incremented by 1: {j}")};
    call_closure_FnMut_mutating_no_moving_out_of_captured_variables(increment);
    // And if the FnMut is allowed, also Fn may be executed
    call_closure_FnMut_mutating_no_moving_out_of_captured_variables(capture_i);
}

fn add_function(a: u32, b: u32) -> u32 {
    a + b
}

fn call_closure_Fn_no_mutating_no_returnig<C: Fn()>(c: C) {
    c();
    c();
}

fn call_closure_FnMut_mutating_no_moving_out_of_captured_variables<C: FnMut()>(mut c: C) {
    c();
    c();
}

fn call_closure_FnOnce_moving_out_captured_variales<C: FnMut()>(mut c: C) {
    c();
    c();
}