fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; //This is the error line. Multiple mutable borrows of `x`.
    *y += 1;
    *z += 1;
    println!("x = {}", x);
}