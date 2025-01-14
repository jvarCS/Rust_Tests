struct A {
    name: String,
}

struct B {
    name: String,
}

impl A {
    fn print(&self) {
        println!("This is struct {}",self.name)
    }
}

impl B {
    fn print(&self) {
        println!("This is struct {}",self.name)
    }
}

enum LetterEnum {
    A(A),
    B(B),
}

// Function to downcast (or match) the LetterEnum
fn describe_letter(letter: LetterEnum) {
    match letter {
        LetterEnum::A(a) => {
            // Downcasting from letter (enum) of type LetterEnum to specific variant A
            a.print();
        }
        LetterEnum::B(b) => {
            // Downcasting from letter (enum) of type LetterEnum to specific variant B
            b.print();
        }
    }
}

fn main() {
    // Create struct instances
    let a = LetterEnum::A(A {
        name: String::from("A"),
    });
    let b = LetterEnum::B(B {
        name: String::from("B"),
    });

    describe_letter(a); 
    describe_letter(b); 
}

/*
Downcasting for enums is a different process compared to downcasting from trait objects. When downcasting with a trait object, Rust doesn't know the concrete type
behind the trait object during compile time. This means the actual downcast operation occurs at runtime and that runtime checks are necessary whenever downcasting is being
performed. However, for enums the process differs in that enums are matched during compile time instead of runtime. Knowing this, it could be said that this program
doesn't perform true downcasting, but instead someting similar in the case of enums.
*/