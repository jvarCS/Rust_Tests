use std::rc::Rc;

fn main() {
    // Create an Rc instance with a value 5
    let a = Rc::new(5);
    println!("Reference count of a: {}", Rc::strong_count(&a)); // 1
    println!("a = {}",a);

    // Create another reference to the same Rc instance
    let b = Rc::clone(&a);
    println!("Reference count of a: {}", Rc::strong_count(&a)); // 2

    // Create another reference to the same Rc instance
    let c = Rc::clone(&a);
    println!("Reference count of a: {}", Rc::strong_count(&a)); // 3
    println!("c = {}",c);

    // Drop one reference
    drop(b);
    println!("Reference count of a: {}", Rc::strong_count(&a)); // 2

    // Drop another reference
    drop(c);
    println!("Reference count of a: {}", Rc::strong_count(&a)); // 1

    // Drop the original reference (a)
    drop(a);
    // Now the memory will be freed, and a will be dropped automatically because the reference count is zero.
}
