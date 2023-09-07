use std::fmt::Display;

struct MyStruct;

impl ToString for MyStruct {
    fn to_string(&self) -> String {
        String::from("MyStruct")
    }
}

struct MySecondStruct;

// Display implements ToString for free
impl Display for MySecondStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MySecondStruct")
    }
}

fn to_string_trait_example() {
    let m = MyStruct;
    print!("{}", m.to_string());
    let ms = MySecondStruct;
    print!("{}", ms.to_string());
}