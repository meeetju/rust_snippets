/// We can implement a trait on a type only if at least one of the trait or the type is local to our crate

/// Example 1 - conflicting implementations

// Cannot have conflicting implementations
trait X {}
// Here we create a blanket implementation for all types that implement Debug.
impl <T: std::fmt::Debug> X for T {}

// So if we add Debug for Y
// #[derive(Debug)] 
struct Y;

// Then this impl will be conflicting
impl X for Y {}



/// Example 2 - implementing trait for different types

// We have a trait
trait Z {}

// Which we can implement for our Type - Q
struct Q;
impl Z for Q {}

// We can also implement for external Types
impl Z for Vec<String> {}