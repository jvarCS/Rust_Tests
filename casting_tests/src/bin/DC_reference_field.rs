use std::any::Any;

trait MyTrait {
    fn print(&self);
    fn as_any(&self) -> &dyn Any; 
}

struct A;

impl MyTrait for A {
    fn print(&self) {
        println!("This is struct A");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct B;

impl MyTrait for B {
    fn print(&self) {
        println!("This is struct B");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Struct holding a reference to a trait object
struct Container<'a> {
    value: &'a dyn MyTrait,  // Reference to a trait object
}

fn main() {
    let a = A;
    let b = B;

    // Create the struct instances
    let container_a = Container { value: &a };
    let container_b = Container { value: &b };

    // Attempt to downcast and print the result
    downcast_and_print(&container_a);
    downcast_and_print(&container_b);
}

// Function to attempt downcasting
fn downcast_and_print<'a>(container: &'a Container<'a>) {
    // Try to downcast the reference to a concrete type
    if let Some(a_ref) = container.value.as_any().downcast_ref::<A>() {
        println!("Successfully downcasted to A");
        a_ref.print(); // This will call A's print method
    } else if let Some(b_ref) = container.value.as_any().downcast_ref::<B>() {
        println!("Successfully downcasted to B");
        b_ref.print(); // This will call B's print method
    } else {
        println!("Downcasting failed");
    }
}
