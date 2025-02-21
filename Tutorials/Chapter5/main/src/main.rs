// fn main() {
// let mut _x: i32 = 5;

// let _r: &mut i32 = &mut _x;

// *_r += 1;
// *_r -= 3;

// println!("Value of x is: {}", _x);
// println!("Value of r is: {}", _r); // r is a reference to x but not an owner. It is immutable by default.


// }



fn main(){


    let mut account:BankAccount = BankAccount{
        account_holder: "John Doe".to_string(),
        balance: 100.0,
    };
    // immutable reference
    account.check_balance();
    // mutable borrow to withtdraw
    account.witdraw(50.3);

}   

struct BankAccount {
    account_holder: String,
    balance: f64,
}

impl BankAccount{

    fn witdraw(&mut self, amount:f64){
        println!("Withdrawing {} from account of {}", amount, self.account_holder);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("The balance of account of {} is {}", self.account_holder, self.balance);
    }

}