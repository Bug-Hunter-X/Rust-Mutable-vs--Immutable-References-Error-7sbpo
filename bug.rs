fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y = 10; // Modify x through y
    println!("x = {}", x); // Output: x = 10

    // This will result in a compile-time error
    // Because z is an immutable reference, we cannot change it
    //*z = 20;  
}