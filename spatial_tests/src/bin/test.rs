use std::rc::Rc;
use std::cell::RefCell;

struct Employee {
    name: String,
    age: u32,
}

fn main() {
    let emp = Rc::new(RefCell::new(Employee {
        name: String::from("Alice"),
        age: 30,
    }));

    // Multiple Rc pointers to the same Employee
    let emp_clone = Rc::clone(&emp);

    // Mutably borrow the Employee and modify it
    emp.borrow_mut().age = 31;
    emp_clone.borrow_mut().name = String::from("Bob");

    // Immutable borrow and access the data
    println!("Employee name: {}", emp.borrow().name);
    println!("Employee age: {}", emp.borrow().age);
}
