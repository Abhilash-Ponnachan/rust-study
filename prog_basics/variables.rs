/*
 Shows how variables and constants are 
 declared and used in Rust
*/
fn main(){
    // variable declaration
    let x = 22;
    let y = 7;
    let pi = x / y;
    println!("Pi = {0}", pi);

    // mutable variables
    let mut z = 5;
    z += 2;
    println!("Next odd = {0}", z);

    // constants
    const UPPER_LIM: u32 = 100;
    let i = UPPER_LIM;
    println!("The upper limit is = {0}", i);

    /// ## shadowing
    let a = 2.3;
    println!("Address of a = {:p}", &a); 
    // address of variable named 'a' = 0x1dc7affba8

    let a = 19;
    println!("Address of a = {:p}", &a); 
    // address of a different variable named 'a' = 0x1dc7affc04

    let a = "alpha";
    println!("Address of a = {:p}", &a); 
    // address of yet another variable named 'a' = 0x1dc7affc58

    // try a mutable variable
    let mut b = 23;
    println!("Address of b = {:p}", &b);
    // address of variable named 'b' = 0x5c74cffbc4

    b = 101;
    println!("Address of b after mutation = {:p}", &b);
    // address of variable named 'b' after changing value = 0x5c74cffbc4

}