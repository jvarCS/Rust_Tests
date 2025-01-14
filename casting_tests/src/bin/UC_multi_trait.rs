// Trait to print the struct type of self
trait Print {
    fn print(&self);
}

// Trait to print hello world
trait Hello {
    fn world(&self);
}

//This combined trait is necessary to call both Print and Hello trait functions from one trait object upcast.
trait PrintAndHello: Print + Hello {}

struct A;

struct B;

impl Print for A {
    fn print(&self) {
        println!("This is struct A");
    }
}

impl Hello for B {
    fn world(&self) {
        println!("Hello world");
    }
}

impl Print for B {
    fn print(&self) {
        println!{"This is struct B"};
    }
}

impl PrintAndHello for B {}

fn main() {
    let a = A; // Define an instance of A
    let a_obj: &dyn Print = &a; // Create a trait object by upcasting &a into trait object of type &dyn Print

    let b = B;  // Define an instance of B

    /*
    The following declaration and initialization of b_obj (a trait object of type &(dyn Print + Hello) would result in a compile-time error.
    This is because when creating trait objects of multiple traits, only one of the traits can be non-auto. Here we are trying to use two
    non-auto traits, Print and Hello, and this error is caught during compilation.

    let b_obj: &(dyn Print + Hello) = &b;

    We want this trait object to have multiple traits so that when we upcast our instance of B into the traits it implements, we will have
    access to the functions of all the traits it implements. (We'll be able to call both print from Print and world from Hello).
    One way to achieve this is by creating a new trait that combines the traits we want to upcast to, in this case Print and Hello.
    This is done by the following:
    */

    let b_obj: &dyn PrintAndHello = &b;  // Create a trait object by upcasting &b into trait object of type &dyn PrintAndHello


    a_obj.print();
    b_obj.world(); 
    b_obj.print();

    // As expected, attempting to call functions from trait objects of a different trait type compared to the one that was upcasted to will also result in error during compile.
    //      a_obj.world();
}


/*
*********************************************  CONCLUSION  *********************************************
The compiler does not allow for trait objects to include more than one non-auto trait. Doing so will result
in compile-time errors. One way around this is to create a combined trait that includes all non-auto traits
you want to be able to access from one single trait object. Then upcast an instance of a struct that 
implements all of these traits into a trait object of type &dyn combined_trait. 
*/