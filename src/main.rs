// core functionality 
// create customer
// create account
// deposit 
// withdraw
// transfer
// check balance 
// customer details 
// close account
// exit

// business rules 
// can't withdraw more than balance 
// can't transfer to non existing account 
// can't transfer to self 
// amount must be positive
// close accounts cannot transact
// customer must exists before creating an account 
// consistent transfers 
// ids must be unique
// error should be proper rust errors 

// bank --owns-- customer --owns-- accounts (saving, current, investments) --owns-- transactions 
use std::{collections::HashMap};

struct Bank {
    customers: HashMap<CustomerId, Customer>,
    accounts: HashMap<AccountId, Account>,
}

// lets implement Bank methods 
// bank must be able to create customers

impl Bank {
    fn new() -> Self { // create an empty customer & account hashmap when bank object is created.
        println!("bank created successfully!");
        return Bank {
            customers: HashMap::new(),
            accounts: HashMap::new()
        };
    }

    fn create_customer(&mut self, name: String, email: String){ // responsible only for creation, the data input should be handled by the main function.    
        let id: CustomerId = CustomerId("123".to_string()); // rust doesn't implicitly converts datatypes for us, so manually converted &str (reference of string baked into main program) to String (stored on heap during runtime) then into customerId.
        let customer = Customer::new(id, name, email);
        
        // self.customers.insert(customer.id, customer); <- WRONG 
        // when you do customer.id -> CustomerId doesn't implement Copy trait so the value is moved out of the struct & struct has no Id.
        // then we try to move entire struct "customer" into vector. 
        // now customer is partially moved. 
        // ┌──────────────────────────┐
        // │ id       → ??? MOVED OUT │
        // │ name     → String        │
        // │ email    → String        │
        // │ accounts → Vec<...>      │
        // └──────────────────────────┘

        // solution, implement Copy Trait for primitive datatypes stored on stack & Clone Trait for compound types like Strings, Vectors etc.
        let customer_id = customer.id.clone();
        self.customers.insert(customer_id, customer);
    }

    fn get_customer(&self, customerid: &CustomerId) -> Option<&Customer> { // functions takes ownership of value if not passed with reference, here we take CustomerId as reference because we only want to look it up 
        // refactor, we just want to return the reference to the found customer. 
        // cli will decide what to do with it. 
        return self.customers.get(customerid)
    }

    // &self X we need mutable reference &mut self correct!
    // read  → &self + get()
    // write → &mut self + get_mut()
    fn update_customer(&mut self, customer_id: &CustomerId, name: String, email: String) {
        // we need mutable reference to the customer object to actually change its content. 
        // let mut customer = self.customers.get(customer_id); // <- WRONG .get return immutable reference & we need &mut reference.
        let customer = self.customers.get_mut(customer_id);
        match customer{
            Some(customer ) => {
                customer.name = name;
                customer.email = email;
            },

            None => {
                println!("customer with id : {:?} doesn't exists", customer_id);
            }
        }
    }

    // delete customer needs reference to CustomerId 
    fn delete_customer(&mut self, customer_id: &CustomerId) {
        // self.customers.remove(customer_id); // .remove() returns Option<V> containing removed value if the key existed in the map. 
        // notice .remove returns Option<V> not Option<&V> that means its throwing away the data, removing it from hashmap and giving us the ownership of removed data 
        match self.customers.remove(customer_id) {
            Some(customer) => println!("deleted customer : {:?}", customer.id),
            None => println!("couldn't find the user with id : {:?}", customer_id),
        }
    }


}

enum AccountType {
    Savings,
    Current,
    Investment,
}

enum TransactionType {
    Deposit,
    Withdrawal,
    TransferOut(AccountId), // what's this ? 
    TransferIn(AccountId), // this too ? 
}

// creating custom types 
#[derive(Debug, Eq, Hash, PartialEq, Clone)] // what does this do and why is it important ? 
// CustomerId isn't hashable or equatable by default, we use macros to make them h-able & e-able.
pub struct CustomerId(String);

#[derive(Debug, Eq, Hash, PartialEq)] 
pub struct AccountId(String);
#[derive(Debug, Eq, Hash, PartialEq)] 
pub struct TransactionId(String);
#[derive(Debug, Eq, Hash, PartialEq)] 
pub struct Money(i64); // keeping i64 instead of f64 to avoid precision errors

#[derive(Debug)] // derieves Debug so it can be printed 
struct Customer {
    id: CustomerId,
    name: String, 
    email: String, 
    accounts: Vec<AccountId>, // customer only holds accountId, actual object owned by bank
}

impl Customer {
    fn new(id: CustomerId, name: String, email: String) -> Self { // constructor for Customer.
        Self {
            id, name, email, accounts: Vec::new(), // initializes new empty accounts vector.
        }
    }
}

struct Account {
    id: AccountId,
    account_type: AccountType,
    owner_id: CustomerId,
    balance: Money, 
    transactions: Vec<Transaction>, // transaction objects owned by accounts 
}

struct Transaction {
    id: TransactionId,
    amount: Money, 
    transaction_type: TransactionType,
}



fn main() {
    let mut bank = Bank::new();
    bank.create_customer("yash".to_string(), "ysonalekar@gmail.com".to_string());
    println!("{:?}", bank.customers); 

    let customerid = CustomerId("123".to_string());
    let customer = bank.get_customer(&customerid);
    match customer{
        Some(customer) => println!("customer found with id : {:?} name : {}", customer.id, customer.name), // {} formatter requires the variable to implement Display Trait (learn later)
        None => println!("customer not found"),
    }

    bank.update_customer(&customerid, "yash sonalekar".to_string(), "yashislearning@gmail.com".to_string());
    match bank.get_customer(&customerid){
        Some(customer) => {
            println!("customer details updated!");
            println!("name : {}", customer.name);
            println!("email : {}", customer.email);
        },
        None => println!("didn't find the customer with id : {:?}", customerid)
    }

    bank.delete_customer(&customerid);
    // this not required because HashMap.remove() throws data, so we can own it since its not owned by the hashmap anymore and consume, discard it.
    // match bank.get_customer(&customerid){
    //     Some(customer) => println!("customer not deleted! {}", customer.name),
    //     None => println!("customer not found!"),
    // }
}
