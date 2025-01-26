fn main() {
    let mut x = 5;
    { // Limit scope for a single mutable borrow
        let y = &mut x;
        *y += 1;
    }
    { // Create a new scope for the second mutable borrow
        let z = &mut x;
        *z += 1; 
    }
    println!("x = {}", x);
}