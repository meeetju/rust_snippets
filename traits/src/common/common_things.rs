pub trait MyBlanketTrait {
    fn who_am_i(&self) {
        println!("I am a blanket impletentation")
    }
}

impl <T> MyBlanketTrait for T {}

pub struct ImportedStruct;