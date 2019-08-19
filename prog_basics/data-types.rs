/*
Demonstrate data types in Rust
 */
fn main(){
    // integer overflow - debug & release mode behaviour
    let mut a: u8 = 255;
    println!("'a' initial value = {0}", a);

    //a += 1;
    //println!("'a' + 1 = {0}", a);
    // debug mode - thread 'main' panicked at 'attempt to add with overflow',
    // release mode - 'a' + 1 = 0

    // Standard numeric operations
    let b = 23;
    let c = 5;
    let d = 3.3;
    let e: u128 = 678;

    println!("{0} + {1} = {2}", b, c, b + c);
    println!("{0} - {1} = {2}", b, c, b - c);
    //println!("{0} * {1} = {2}", b, d, b * d);
    println!("{0} / {1} = {2}", b, c, b / c);
    //println!("{0} / {1} = {2}", b, d, b / d);
    println!("{0} % {1} = {2}", b, c, b % c);
    //println!("{0} % {1} = {2}", b, d, b % d);
    println!("{0} / {1} = {2}", b, e, b / e);

    let f = true;
    println!("Opposite of {0} is {1}", f, !f);

    let g = 'A';
    let h = '\u{41}';   // unicode value of 'A' in Hex
    let i = '\u{03A3}'; // unicode value of Greek Zigma in Hex
    println!("{0}, {1}, {2}", g, h, i); 
    //A, A, Σ    

}