use std::any::Any;

trait Letter: Any {
    fn print(&self);
    fn as_any(&self) -> &dyn Any;  // A method that returns a reference to `Any`
}

struct A;
struct B;

impl Letter for A {
    fn print(&self) {
        println!("This is struct A");
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Letter for B {
    fn print(&self) {
        println!("This is struct B");
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let a: Box<dyn Letter> = Box::new(A);
    let b: Box<dyn Letter> = Box::new(B);

    // Example of unsafe downcasting (casting a trait object to a concrete type)
    unsafe {
        // Attempt to downcast `a` to `A`
        if let Some(a_ref) = downcast_ref::<A>(&*a) {
            println!("Downcasted to struct A");
            a_ref.print();
        } else {
            println!("Failed to downcast.");
        }

        // Attempt to downcast `b` to struct B
        if let Some(b_ref) = downcast_ref::<B>(&*b) {
            println!("Downcasted to struct B");
            b_ref.print(); 
        } else {
            println!("Failed to downcast.");
        }

        // Attempt to downcast 'a' to struct B, which should fail
        if let Some(b_ref) = downcast_ref::<B>(&*a) {
            println!("Downcasted to struct B");
            b_ref.print();  
        } else {
            println!("Failed to downcast.");
        }
    }
}

// Unsafe downcast function using `Any`
unsafe fn downcast_ref<T: 'static>(ptr: &dyn Letter) -> Option<&T> {
    if let Some(t) = ptr.as_any().downcast_ref::<T>() {
        Some(t)
    } else {
        None
    }
}
