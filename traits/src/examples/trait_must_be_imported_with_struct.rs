use crate::common::common_things::{ImportedStruct, MyBlanketTrait};

fn example() {
    let i_s = ImportedStruct;
    i_s.who_am_i(); // This method appears only when the trait MyBlanketTrait is imported
}