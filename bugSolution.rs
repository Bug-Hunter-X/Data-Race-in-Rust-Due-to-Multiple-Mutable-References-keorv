fn main() {
    let mut x = 5;
    { // New scope
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    } // y goes out of scope here
    { // New Scope
        let z = &mut x; // z is a mutable reference to x
        *z = 20; // Modify x through z
    }// z goes out of scope here

    println!("x is: {}", x); // Prints 20
}

//Another approach using clone:

fn main() {
    let mut x = 5;
    let mut y = x;
    let mut z = x;
    y = 10;
    z = 20;
    println!("x is: {}", x); //Prints 5, y is 10, z is 20 
}