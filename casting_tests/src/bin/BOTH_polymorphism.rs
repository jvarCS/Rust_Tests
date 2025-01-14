use std::any::Any;

// Disp trait
trait Disp {
    fn getIVal(&self) -> u32;
    fn getFVal(&self) -> f32;
    fn getCVal(&self) -> char;

    // A method to attempt to downcast the trait object
    fn as_any(&self) -> &dyn Any;
}

// A struct
struct A {
    iVal: u32,
    fVal: f32,
    cVal: char,
}

impl A {
    fn new(iVal: u32, fVal: f32, cVal: char) -> Self {
        A {
            iVal: iVal,
            fVal: fVal,
            cVal: cVal,
        }
    }
}

impl Disp for A {
    fn getIVal(&self) -> u32 {
        self.iVal
    }

    fn getFVal(&self) -> f32 {
        self.fVal
    }

    fn getCVal(&self) -> char {
        self.cVal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// B struct
struct B {
    iVal: u32,
    fVal: f32,
}

impl B {
    fn new(iVal: u32, fVal: f32) -> Self {
        B {
            iVal: iVal,
            fVal: fVal,
        }
    }
    
    fn printB(&self) {
        println!("I'm B");
    }
}

impl Disp for B {
    fn getIVal(&self) -> u32 {
        self.iVal
    }

    fn getFVal(&self) -> f32 {
        self.fVal
    }

    fn getCVal(&self) -> char {
        'B'
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    // Create struct instances
    let a = A::new(1,1.1,'A');
    let b = B::new(2,2.2);

    // Box the trait objects, upcast to type dyn Disp
    let structs: Vec<Box<dyn Disp>> = vec![
        Box::new(a),
        Box::new(b),
    ];

    // Downcast back to concrete types
    for s in structs {
        if let Some(a_ref) = s.as_any().downcast_ref::<A>() {
            println!("Downcasted to struct A");
            println!("i val: {}",a_ref.getIVal());
            println!("f val: {}",a_ref.getFVal());
            println!("c val: {}",a_ref.getCVal());
        } else if let Some(b_ref) = s.as_any().downcast_ref::<B>() {
            println!("Downcasted to struct B");
            println!("i val: {}",b_ref.getIVal());
            println!("f val: {}",b_ref.getFVal());
            b_ref.printB();
        } else {
            println!("Error");
        }
    }
}


/*
In this test we perform upcasting and downcasting. The upcast occurs after we create instances of the structs
and place them in a vector of type dyn Disp. That is, a vector whose type is the trait that the structs implement. 
This way we can utilize polymorphism to group all struct objects that implement a trait type in one single vector,
downcast each element in the vector to its original concrete type, and call its respective functions.
*/
