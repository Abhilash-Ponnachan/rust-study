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

    let scores = [67, 84, 47, 56, 78];
    let mut mean_score = 0;
    for s in &scores{ // use 'slice` or '.iter()'
        mean_score += s;
    }
    mean_score /= scores.len();
    println!("Mean score = {0}", mean_score);
    // Mean score = 66
}