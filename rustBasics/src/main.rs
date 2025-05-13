struct User {
    name: String,
}

fn print_user(user: &User) {
    println!("the name of the user is: {}", user.name);
}

fn mutable_reference() {
    let mut x: i32 = 1;
    let x_ref = &mut x;

    *x_ref += 1;

    println!("The value of x_ref is: {}", x_ref);
}
fn main() {
    let user = User {
        name: "obinna".to_string(), //string literal
    };

    print_user(&user); // ownership of user stransfered to print_user

    println!("the user is: {}", user.name);
    mutable_reference();
}

// We use colon (:) to define the type of a variable in Rust.
// We have two types of strings in Rust: String and str.
// Struct is a complex type and is conceptually similar to a class in other languages.
// Variables are immutable by default in Rust unless you add the mut keyword after the definition.
// Constants are defined using const in Rust.
// We use let to define a variable in Rust.
// String is heap-allocated and growable in Rust, similar to strings in other programming languages.
// str, often used as &str, is a read-only string slice.
// Structs in Rust do not have methods directly attached to them.
// To attach methods, an impl block is used. For example:
//     struct User {};
//     impl User {
//         fn greet(&self) { ... }
//     }
// Functions are defined using the fn keyword, e.g., fn create_user() {}

// In Rust, each value has a single owner.
// When ownership is transferred (moved), the original variable can no longer be used.
// e.g
/*
    let a = String::from("hello");
    let b = a; // ownership of the String moves to b
    println!("{}", a); // ❌ compile error
*/

// Rust allows borrowing with references (&) without transferring ownership.
// Mutable references (&mut) allow modifying the value.
/*
   fn greet(name: &String) {
       println!("Hello, {}", name);
   }
*/

// You Cannot Have Both Mutable and Immutable References at the Same Time: Prevents data races at compile time.
//e.g
/*
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // ❌ cannot borrow as mutable while immutable refs exist
*/

// Shadowing Is Allowed: You can redefine a variable with the same name using let. e.g
/*
    let x = 5;
    let x = x + 1; // shadows previous x
*/

// Pattern Matching with match:  Powerful alternative to switch-case. e.g
/*
        let number = 2;
        match number {
            1 => println!("one"),
            2 => println!("two"),
            _ => println!("something else"),
}
*/

// Concurrency Is Safe and Built In
// Rust uses modules (mod), crates, and packages for organizing code.
// External packages are added using Cargo (Rust's package manager).
// Rust has no garbage collector - Memory is managed at compile-time using ownership rules.

// Traits define shared behavior. e.g
/*
   trait Greet {
       fn greet(&self);
   }

   struct Person;

   impl Greet for Person {
       fn greet(&self) {
           println!("Hi!");
       }
   }
*/
