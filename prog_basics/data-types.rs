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

    let mut rec: (&'static str, i32, f32) = ("Alan", 1001, 78.6);
    rec.1 = 1002;
    println!("Record is {:?}", rec);
    // Record is ("Alan", 1002, 78.6)

    let (name, _ , weight) = rec;
    println!("{0} weighs {1} Kg!", name, weight);
    // Alan weighs 78.6 Kg!

    let mut scores = [87, 67, 48, 56, 73];
    scores[0] = 92;
    scores[scores.len() - 1] = 78; // using len() method to get the length
    println!("New scores are {:?}", scores);
    // New scores are [92, 67, 48, 56, 78]

    let mut sum = 0;
    // using 'for' to iterate over the array
    for i in scores.iter(){
        sum += i;
    }
    println!("Total score = {0}", sum);

    let vow: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    // note type declaration uses ';'
}