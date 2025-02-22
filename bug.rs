fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y = 10; // Modify x through y
    *z = 20; // Modify x through z
    println!("x is: {}", x); // this will print 20 because the last modification is counted 
}