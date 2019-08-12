/*
Demonstrate data types in Rust
 */
fn main(){
    // integer overflow - debug & release mode behaviour
    let mut a: u8 = 255;
    println!("'a' initial value = {0}", a);

    a += 1;
    println!("'a' + 1 = {0}", a);
    // debug mode - thread 'main' panicked at 'attempt to add with overflow',
    // release mode - 'a' + 1 = 0
}