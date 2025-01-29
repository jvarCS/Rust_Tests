use std::io;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;
use std::fmt;

// cause dangling pointer (manually drop) - { Test dropping at different points and see how rust responds }
// multiple references on same employee object, RR, RW, WW... - { }
// multiple hashmaps to see if rc is incremented - { Pop an element in one which causes it to drop and see how it effects the other hashmap }
// investigate rc overhead

struct Employee {
    name: String,
    id: u32,
    age: u32,
}

impl Employee {
    fn new(name: String, id: u32, age:u32) -> Employee {
        Employee { name, id, age }
    }
    fn changeName(&mut self, name: String) {
        self.name = name;
    }
    fn changeAge(&mut self, age: u32) {
        self.age = age;
    }
    fn changeID(&mut self, id: u32) {
        self.id = id;
    }
    fn getName(&self) -> &str {
        &self.name
    }
    fn getAge(&self) -> u32 {
        self.age
    }
    fn getID(&self) -> u32 {
        self.id
    }
}

impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Employee Name: {}, ID: {}, Age: {}", self.name, self.id, self.age)
    }
}

impl Drop for Employee {
    fn drop(&mut self) {
        println!("Dropping Employee struct with name: {}", self.name);
    }
}

fn main() {
    let mut done = false;
    let mut employees: HashMap<u32, Rc<*mut Employee>> = HashMap::new(); 
    let mut employees2: HashMap<u32, Rc<*mut Employee>> = HashMap::new(); 
    let mut empStore: HashMap<u32, Rc<Employee>> = HashMap::new(); 
    while !done {
        let mut input = String::new();
        println!("1 to add employee");
        println!("2 to change employee info");
        println!("3 to delete employee");
        println!("4 to print employee info");
        println!("5 to exit");

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice = input.trim().parse().expect("WRONG");

        match choice {
            1 => {  // Add Employee
                let mut name = String::new();
                println!("Enter name");
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let mut name = name.trim().to_string();
                //println!("NAME IS: {name}");
                let mut age = String::new();
                println!("Enter age");
                io::stdin().read_line(&mut age).expect("Failed to read line");
                let eAge: u32 = age.trim().parse().expect("Please enter a valid number");

                let mut id = String::new();
                println!("Enter id");
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let eID: u32 = id.trim().parse().expect("WRONG");
                
                // Create object outside of RC stuff with raw pointer and see how compiler reacts. UNSAFE   -    In meeting                
                // Employee object created outside of RC/RefCell creation.  - WORKS ( STEP 1 )
                let mut emp1 = Employee::new(name,eID,eAge);
                println!("employee name: {}, employee age: {}, employee id: {}",emp1.getName(),emp1.getAge(), emp1.getID());
                let emPtr: *mut Employee = &mut emp1;

                // Create RC/RefCell with raw pointer   - WORKS     ( STEP 2 )
                let temp = Rc::new(emPtr); 
                let emp_rc = Rc::new(emp1);

                let temp2 = temp.clone();   // Clone raw pointer into new Rc which increments reference count of initial employee object
                employees.insert(eID, temp);        // Insert both Rc's into different hashmaps
                employees2.insert(eID, temp2);
                empStore.insert(eID, emp_rc);
                //START HERE    -   This is process to get pointer from hashmap and use it to print data. WORKS HERE    ( CALL THIS PROCESS A )
                // unsafe {
                //     if let Some(emp_rc) = employees.get(&eID) { 
                //         if let Some(emp_ref) = emp_rc.as_mut() {
                //             println!("{}",emp_ref);
                //             println!("Name: {}", emp_ref.getName());
                //             println!("ID: {}", emp_ref.getID());
                //             println!("Age: {}", emp_ref.getAge());
                //         }
                //         // Display reference count after printing employee info
                //         println!("Reference count for emp_rc {}: {}", eID, Rc::strong_count(&emp_rc));
                //     }
                // }

                // Drop trait   -   Modify output 

                // Display reference count after adding an employee
                println!("Reference count for employee {}: {}", eID, Rc::strong_count(&employees[&eID]));
            },
            2 => {  // Change employee info
                println!("Enter employee id");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let eId: u32 = id.trim().parse().expect("WRONG");

                println!("1 to change name");
                println!("2 to change age");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let inp: u32 = input.trim().parse().expect("WRONG");

                if let Some(emp_rc) = employees.get(&eId) {
                    unsafe {    // Unsafe rust begins here. This whole block is essentially getting the ptr from the hashmap and then borrowing the ptr as a ref to call
                        if let Some(emp_ref) = emp_rc.as_mut() {
                            match inp {
                                1 => {
                                    let mut name = String::new();
                                    println!("Enter name");
                                    io::stdin().read_line(&mut name).expect("Failed to read line");
                                    let mut name = name.trim().to_string();
                                    emp_ref.changeName(name);                                                          
                                },
                                2 => {
                                    let mut age = String::new();
                                    println!("Enter age");
                                    io::stdin().read_line(&mut age).expect("Failed to read line");
                                    let eAge: u32 = age.trim().parse().expect("WRONG");
                                    emp_ref.changeAge(eAge);
                                },
                                _ => { println!("WRONG"); }
                            }
                        }
                    }
                        // Display reference count after modifying employee info
                        println!("Reference count for employee {}: {}", eId, Rc::strong_count(&emp_rc));
                } else {
                    println!("ID not found");
                }
            },
            3 => {  // Delete employee
                println!("Enter employee id");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let eId: u32 = id.trim().parse().expect("WRONG");

                if employees.remove(&eId).is_none() {
                    println!("ID not found");
                } else {
                    println!("Employee {} removed", eId);
                }

                // Display reference count after deletion
                if let Some(emp_rc) = employees.get(&eId) {
                    println!("Reference count for employee {}: {}", eId, Rc::strong_count(&emp_rc));
                }
            },
            4 => {  // Print employee info
                println!("Enter employee id");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let eId: u32 = id.trim().parse().expect("WRONG");

                unsafe {    
                    if let Some(emp_rc) = employees.get(&eId) { 
                        if let Some(emp_ref) = emp_rc.as_mut() {
                            // println!("{}",emp_ref);
                            println!("Name: {}", emp_ref.getName());
                            println!("ID: {}", emp_ref.getID());
                            println!("Age: {}", emp_ref.getAge());
                        }
                        // Display reference count after printing employee info
                        println!("Reference count for emp_rc {}: {}", eId, Rc::strong_count(&emp_rc));
                    }
                    else {
                        println!("ID not found in employees");
                    }
                    if let Some(emp_rc2) = employees2.get(&eId) {
                        if let Some(emp_ref) = emp_rc2.as_mut() {
                            // println!("{}",emp_ref);
                            println!("Name: {}", emp_ref.getName());
                            println!("ID: {}", emp_ref.getID());
                            println!("Age: {}", emp_ref.getAge());
                        }
                        
                        // Display reference count after printing employee info
                        println!("Reference count for emp_rc2 {}: {}", eId, Rc::strong_count(&emp_rc2));
                    }
                    else {
                        println!("ID not found in employees2")
                    }
                }
            },
            5 => {  // Exit
                done = true;
            },
            _ => { println!{"WRONG"}; }
        }
    }
}