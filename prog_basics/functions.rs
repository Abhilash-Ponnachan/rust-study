// 'main' enrtry-point function
fn main(){
    say_hello();
    say_hello_to("Alan");

    let nums = [10, 20, 30, 40];
    println!("Mean = {0}", get_mean(&nums));
    // Mean = 25
}

// function with no parameters
fn say_hello(){
    println!("Hola!");
}

// function with parameters
fn say_hello_to(name: &str){
    // paramter type is 'str slice'
    println!("Hello {0}!", name);
}

// function with params & return type
// param type = slice of array
// return type = i32
fn get_mean(nums: &[i32]) -> i32{
    let mut sum = 0;
    for i in nums.iter(){
        sum += i;
    }
    return sum / nums.len() as i32;
}