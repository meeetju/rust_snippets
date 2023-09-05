trait MyInterface {
    fn say_concrete_hi(&self) -> &str;
    fn say_something(&self) -> &str;
    fn say_default(&self) -> &str {
        "Default!"
    }
}

struct MyStruct;

// not all trait items implemented, missing: `say_concrete_hi`, `say_something`
impl MyInterface for MyStruct {
    fn say_concrete_hi(&self) -> &str {
        "Hi!"
    }

    fn say_something(&self) -> &str {
        "Something!"
    }
}

struct MySecondStruct {
    pub first_field: String
}

impl MyInterface for MySecondStruct {
    fn say_concrete_hi(&self) -> &str {
        "Hi second!"
    }

    fn say_something(&self) -> &str {
        &self.first_field
    }

    fn say_default(&self) -> &str {
        "Default second!"
    }
}

pub fn example() {
    let my_struct = MyStruct;

    println!("{}", my_struct.say_concrete_hi());
    println!("{}", my_struct.say_something());
    println!("{}", my_struct.say_default());

    let my_second_struct = MySecondStruct{first_field: "lol".to_owned()};

    println!("{}", my_second_struct.say_concrete_hi());
    println!("{}", my_second_struct.say_something());
    println!("{}", my_second_struct.say_default());

}