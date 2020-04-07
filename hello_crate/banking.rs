pub mod bank{
    pub mod accounts{
        #[derive(Debug)]
        pub enum AccType {
            Current,
            Savings,
            Investment
        }
        
        pub struct Account{
            number: u64,
            pub amount: f32
        }
        
        impl Account{
            pub fn create(number: u64) -> Account{
                Account{
                    number: number,
                    amount: 0.0
                } 
            }
            
            pub fn deposit(&mut self, amt: f32){
                self.amount += amt;  
            }
            
            pub fn withdraw(&mut self, amt: f32) -> bool {
                if amt <= self.amount{
                   self.amount -= amt;
                   return true
                }
                return false  
            }
        }
        
    }
        
    mod customers{
        
        struct Customer{
            id: String,
            name: String,
            address: Address
            // account: Account
        }
        
        struct Address{
            house: String,
            street: String,
            city: String,
            postcode: String
        }
    }
}

/*
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
*/