trait SayHi {
    fn say_hi(&self) {}
}

struct FirstOne;

impl SayHi for FirstOne {
    fn say_hi(&self) {
        println!("Hi from FirstOne!");
    }
}

struct SecondOne;

pub fn example_pass_struct_implementing_trait() {
    let f_one = FirstOne;
    function_accepting(Box::new(f_one));

    // the trait bound `SecondOne: SayHi` is not satisfied
    // the trait `SayHi` is implemented for `FirstOne`
    // required for the cast from `SecondOne` to the object type `dyn SayHi

    // let s_one = SecondOne;
    // function_accepting(Box::new(s_one));
}

fn function_accepting(in_struct: Box<dyn SayHi>) {
    in_struct.say_hi();
}

pub fn example_return_struct_implementing_trait() {
    let my_struct = function_returning();

    my_struct.say_hi();
}

fn function_returning() -> impl SayHi {
    FirstOne

    // the trait bound `SecondOne: SayHi` is not satisfied
    // the trait `SayHi` is implemented for `FirstOne`

    // SecondOne
}