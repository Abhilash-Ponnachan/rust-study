fn main(){
    //loop
    println!();
    let mut i = 1;
    loop{
        print!("{},", i);
        i += 1;
        if i > 10 {
            break;
        }
    };
    // repeats the above block 10 times
    println!();
    // 1,2,3,4,5,6,7,8,9,10,
}