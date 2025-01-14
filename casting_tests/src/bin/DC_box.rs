use std::any::Any;

trait Letter {
    fn as_any(&self) -> &dyn Any;
}

struct A;
impl Letter for A {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let a: Box<dyn Letter> = Box::new(A);

    // Downcasting the Box<dyn Animal> to Box<Dog>
    if let Some(a_ref) = a.as_any().downcast_ref::<A>() {
        println!("Successfully downcasted to struct A.");
    } else {
        println!("Failed to downcast.");
    }
}