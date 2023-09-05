mod common;
mod examples;

fn main() {
    println!("Example blanket implementation");
    examples::blanket_implementation::example();
    println!("------------------------------");
    println!("Example function accepting type that implements trait");
    examples::traits_definitions::example_pass_struct_implementing_trait();
    println!("------------------------------");
    println!("Example function returning type that implements trait");
    examples::traits_definitions::example_return_struct_implementing_trait();
    println!("------------------------------");
    println!("Example with default implementation vs concrete");
    examples::default_implementation::example();
    println!("------------------------------");
}
