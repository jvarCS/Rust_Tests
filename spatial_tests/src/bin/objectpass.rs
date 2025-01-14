use std::io;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

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

fn main() {
    let mut done = false;
    let mut employees: HashMap<u32, Rc<RefCell<Employee>>> = HashMap::new(); 
    let mut employees2: HashMap<u32, Rc<RefCell<Employee>>> = HashMap::new(); 
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
            1 => {
                let mut name = String::new();
                println!("Enter name");
                io::stdin().read_line(&mut name).expect("Failed to read line");

                let mut age = String::new();
                println!("Enter age");
                io::stdin().read_line(&mut age).expect("Failed to read line");
                let eAge: u32 = age.trim().parse().expect("Please enter a valid number");

                let mut id = String::new();
                println!("Enter id");
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let eID: u32 = id.trim().parse().expect("WRONG");

                let temp = Rc::new(RefCell::new(Employee::new(name, eID, eAge))); 
                let temp2 = temp.clone();   // HERE NEW TEST, pass in temp reference instead of actual object (temp)
                employees.insert(eID, temp);        // HERE NEW TEST, what is clone actually doing??
                employees2.insert(eID, temp2);
                // employees2.insert(eID, temp); - { Doesnt work cuz move }

                // Display reference count after adding an employee
                println!("Reference count for employee {}: {}", eID, Rc::strong_count(&employees[&eID]));
            },
            2 => {
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
                    let mut emp = emp_rc.borrow_mut(); // Mutably borrow the employee

                    match inp {
                        1 => {
                            let mut name = String::new();
                            println!("Enter name");
                            io::stdin().read_line(&mut name).expect("Failed to read line");
                            emp.changeName(name);
                        },
                        2 => {
                            let mut age = String::new();
                            println!("Enter age");
                            io::stdin().read_line(&mut age).expect("Failed to read line");
                            let eAge: u32 = age.trim().parse().expect("WRONG");
                            emp.changeAge(eAge);
                        },
                        _ => { println!("WRONG"); }
                    }

                    // Display reference count after modifying employee info
                    println!("Reference count for employee {}: {}", eId, Rc::strong_count(&emp_rc));
                } else {
                    println!("ID not found");
                }
            },
            3 => {
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
            4 => {
                println!("Enter employee id");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let eId: u32 = id.trim().parse().expect("WRONG");

                // CHANGE HERE - We now check if there is Some() in employees2 instead of employees, ensuring the use of the second hashmap
                if let Some(emp_rc) = employees.get(&eId) { 
                    let emp = emp_rc.borrow(); // Borrow the employee immutably
                    println!("Name: {}", emp.getName());
                    println!("ID: {}", emp.getID());
                    println!("Age: {}", emp.getAge());
                    
                    // Display reference count after printing employee info
                    println!("Reference count for emp_rc {}: {}", eId, Rc::strong_count(&emp_rc));
                }
                else {
                    println!("ID not found in employees");
                }
                if let Some(emp_rc2) = employees2.get(&eId) {
                    let emp2 = emp_rc2.borrow(); // Borrow the employee immutably
                    println!("Name: {}", emp2.getName());
                    println!("ID: {}", emp2.getID());
                    println!("Age: {}", emp2.getAge());
                    
                    // Display reference count after printing employee info
                    println!("Reference count for emp_rc2 {}: {}", eId, Rc::strong_count(&emp_rc2));
                }
                else {
                    println!("ID not found in employees2")
                }
            },
            5 => {
                done = true;
            },
            _ => { println!{"WRONG"}; }
        }
    }
}