// 'main' enrtry-point function
fn main(){
    say_hello();
    say_hello_to("Alan");

    let nums = [10, 20, 30, 40, 50];
    let mean = get_mean(&nums);
    println!("Mean = {0}", mean);
    // Mean = 30

    let root = {
        let r = mean as f32; //note: 'mean' from outer scope
        r.sqrt() // no semicolon
    };
    // value of 'r.sqrt()' gets assigned to 'root'
    println!("Root = {0}", root);
    // Root = 5.477226
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