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
    let mut a = 5;
    a += 2;
    println!("Next odd = {0}", a);
}