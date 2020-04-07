extern crate banking;

fn main(){
    
    use banking::bank::accounts::Account;

    let mut acc = Account::create(100120013001);

    acc.deposit(2000.0);

    println!("account balance is {}", acc.amount);
}

/*
# Where library.rlib is the path to the compiled library, assumed that it's
# in the same directory here:
$ rustc executable.rs --extern rary=library.rlib 
$ ./executable
*/