// ';' after module name => load from a file 'bank.rs'
mod banking;

fn main() {

    // import 'Account' into scope
    use banking::bank::accounts::Account;

    let mut acc = Account::create(100120013001);

    acc.deposit(2000.0);

    println!("account balance is {}", acc.amount);
}
