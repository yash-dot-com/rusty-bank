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
}
