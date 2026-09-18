// core functionality
// create customer
// deposit
// withdraw
// create account
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

// // bank --owns-- customer --owns-- accounts (saving, current, investments) --owns-- transactions
// use std::collections::HashMap;

// // mutable reference of bank will allow mutable access to all the variables owned by it.
// struct Bank {
//     customers: HashMap<CustomerId, Customer>,
//     accounts: HashMap<AccountId, Account>,
//     next_customer_id: u64,
//     next_account_id: u64,
// }

// // lets implement Bank methods
// // bank must be able to create customers

// impl Bank {
//     // revised.
//     // constructor for bank object
//     fn new() -> Self {
//         // create an empty customer & account hashmap when bank object is created.
//         println!("bank created successfully!");
//         return Bank {
//             customers: HashMap::new(),
//             accounts: HashMap::new(),
//             next_customer_id: 1,
//             next_account_id: 1,
//         };
//     }

//     //
//     fn create_customer(&mut self, name: String, email: String) {
//         // responsible only for creation, the data input should be handled by the main function.
//         // let id: CustomerId = CustomerId("123".to_string()); // rust doesn't implicitly converts datatypes for us, so manually converted &str (reference of string baked into main program) to String (stored on heap during runtime) then into customerId.
//         let id = CustomerId(self.next_customer_id.to_string());
//         self.next_customer_id += 1;

//         // cloned the id value here to be able to use later.
//         let customer = Customer::new(id.clone(), name, email);

//         // self.customers.insert(customer.id, customer); <- WRONG
//         // when you do customer.id -> CustomerId doesn't implement Copy trait so the value is moved out of the struct & struct has no Id.
//         // then we try to move entire struct "customer" into vector.
//         // now customer is partially moved.
//         // ┌──────────────────────────┐
//         // │ id       → ??? MOVED OUT │
//         // │ name     → String        │
//         // │ email    → String        │
//         // │ accounts → Vec<...>      │
//         // └──────────────────────────┘

//         // solution, implement Copy Trait for primitive datatypes stored on stack & Clone Trait for compound types like Strings, Vectors etc.
//         // let customer_id = customer.id.clone();

//         // cloned the id previously to use here.
//         self.customers.insert(id, customer);
//     }

//     // need only read access
//     fn get_customer(&self, customerid: &CustomerId) -> Option<&Customer> {
//         // functions takes ownership of value if not passed with reference, here we take CustomerId as reference because we only want to look it up
//         // refactor, we just want to return the reference to the found customer.
//         // cli will decide what to do with it.
//         return self.customers.get(customerid);
//     }

//     // revised.
//     // &self <- WRONG, we need mutable reference &mut self.
//     // read  → &self + get()
//     // write → &mut self + get_mut()
//     fn update_customer(&mut self, customer_id: &CustomerId, name: String, email: String) {
//         // we need mutable reference to the customer object to actually change its content.
//         // let mut customer = self.customers.get(customer_id); // <- WRONG .get return immutable reference & we need &mut reference.
//         let customer = self.customers.get_mut(customer_id);
//         match customer {
//             Some(customer) => {
//                 customer.name = name;
//                 customer.email = email;
//             }

//             None => {
//                 println!("customer with id : {:?} doesn't exists", customer_id);
//             }
//         }
//     }

//     // delete customer needs reference to CustomerId
//     fn delete_customer(&mut self, customer_id: &CustomerId) {
//         // self.customers.remove(customer_id); // .remove() returns Option<V> containing removed value if the key existed in the map.
//         // notice .remove returns Option<V> not Option<&V> that means its throwing away the data, removing it from hashmap and giving us the ownership of removed data
//         match self.customers.remove(customer_id) {
//             Some(customer) => println!("deleted customer : {:?}", customer.id),
//             None => println!("couldn't find the user with id : {:?}", customer_id),
//         }
//     }

//     // customer CRUD done.
//     // lets build account CRUD
//     // fn create_account(&mut self, customer_id: &CustomerId, account_type: AccountType, initial_balance: Money = 0, ) {
//     //     // check if customer exists
//     //     // generate new account id
//     //     // create new account
//     //     // store account in self.hashmap & customer.accounts vector
//     //     // MAJOR OWNERSHIP REFACTOR
//     //     // bank owns the accountid field for hashmap
//     //     // customer only keeps the reference because bank is the source of truth.
//     //     match self.customers.get_mut(customer_id) {
//     //         Some(customer) => {
//     //             let account_id = AccountId("123".to_string());
//     //             let customer_id = customer.id.clone(); // cloning customer id to store in hashmap without taking ownership from customer object.
//     //             let account = Account::new(account_id.clone(), account_type, customer_id);

//     //             customer.accounts.push(account_id.clone()); // account_id already moved.
//     //             self.accounts.insert(account_id, account); // again account_id already moved.
//     //         }
//     //         None => println!("Customer doesn't exists, Account cannot be created.")
//     //     }
//     // }

//     // refactoring create_account() fn to return AccountId.
//     // returns &AccountId because I don't want to move accountid out of the account object.
//     fn create_account(
//         &mut self,
//         customer_id: &CustomerId,
//         account_type: AccountType,
//     ) -> Option<AccountId> {
//         match self.customers.get_mut(customer_id) {
//             Some(customer) => {
//                 // let account_id = AccountId("123".to_string());
//                 let account_id = AccountId(self.next_account_id.to_string());
//                 self.next_account_id += 1;

//                 let account = Account::new(account_id.clone(), account_type, customer.id.clone());

//                 customer.accounts.push(account_id.clone());
//                 // cloning account_id before accounts hashmap owns the account_id
//                 let returned_id = account_id.clone();

//                 self.accounts.insert(account_id, account);
//                 Some(returned_id)

//                 // ownership flow
//                 //                     ┌── clone → Account.id
//                 //                     │
//                 // account_id ─────────┼── clone → Customer.accounts
//                 //                     │
//                 //                     ├── clone → returned_id
//                 //                     │
//                 //                     └── move → Bank.accounts HashMap key
//             }
//             None => {
//                 println!("customer doesn't exists, account cannot be created.");
//                 None
//             }
//         }
//     }

//     fn get_account(&self, account_id: &AccountId) -> Option<&Account> {
//         // check if account exists, return &Account
//         // else return None
//         // match self.accounts.get(account_id) {
//         //     Some(account) => Some(account),
//         //     None => None,
//         // }

//         // also the hashmap returns Option<&Account> so we can just put
//         self.accounts.get(account_id)
//     }

//     fn get_account_owner(&self, account_id: &AccountId) -> Option<&Customer> {
//         // check if account exists
//         // if yes then return its customer
//         // if no then return none.
//         match self.accounts.get(account_id) {
//             Some(account) => match self.customers.get(&account.owner_id) {
//                 Some(owner) => Some(owner),
//                 None => None,
//             },
//             None => None,
//         }
//     }

//     // fn to list customer's all accounts must return Option<Vec<Account>>
//     fn get_all_accounts(&self, customer_id: &CustomerId) -> Option<Vec<&Account>> {
//         // bank owns account
//         // get_all_accounts() -> borrows accounts -> Vec<&Account>
//         // cli iterates and prints
//         match self.customers.get(customer_id) {
//             Some(customer) => {
//                 println!("customer found: {}", customer.name);
//                 let mut accounts = Vec::new();

//                 // if no account id we return empty vector

//                 for account_id in &customer.accounts {
//                     match self.accounts.get(account_id) {
//                         Some(account) => accounts.push(account),
//                         None => {}
//                     }
//                 }
//                 // return Some(accounts) after constructing the vector
//                 Some(accounts)
//             }
//             None => None,
//         }
//     }

//     fn close_account(&mut self, account_id: &AccountId) {
//         // first lets check if account exists in the bank
//         // if yes then lets remove it from bank
//         // then we will proceed to remove from it from owner's account id vector.
//         //     match self.accounts.get(account_id) {
//         //         Some(account) => {
//         //             // if the account exists,
//         //             let owner_id = account.owner_id.clone();
//         //             let owner = self.customers.get_mut(&owner_id);
//         //             match owner {
//         //                 Some(owner) => {
//         //                     owner.accounts.retain(|id| id != account_id);
//         //                 },
//         //                 None => {
//         //                     println!("account has no owner!")
//         //                 }
//         //             }

//         //             match self.accounts.remove(account_id) {
//         //                 Some(_) => {
//         //                     println!("account closed successfully!");
//         //                 },
//         //                 None => {
//         //                     println!("could not close account");
//         //                 }
//         //             }

//         //         },
//         //         None => {
//         //             println!("account doesn't exists!");
//         //         }
//         // }

//         // clever code
//         // remove(account_id)
//         //     ↓
//         // Option<Account>
//         //     ↓
//         // Some(account)
//         //     ↓
//         // account.owner_id
//         //     ↓
//         // find customer
//         //     ↓
//         // retain account_id
//         match self.accounts.remove(account_id) {
//             Some(account) => {
//                 let owner_id = account.owner_id;
//                 match self.customers.get_mut(&owner_id) {
//                     Some(owner) => {
//                         owner.accounts.retain(|id| id != account_id);
//                         println!("account closed successfully!");
//                     }
//                     None => {
//                         println!("owner not found!");
//                     }
//                 }
//             }
//             None => {
//                 println!("account not found!");
//             }
//         }
//         // how its working :-
//         // Bank.accounts
//         //     │
//         //     │ remove(account_id)
//         //     ↓
//         // Account  ← you own it
//         //     │
//         //     │ move owner_id
//         //     ↓
//         // Customer
//         //     │
//         //     │ retain()
//         //     ↓
//         // remove AccountId from customer's Vec
//     }

//     // day 3 - implementing bank operations
//     // fn deposit
//     fn deposit(&mut self, account_id: &AccountId, amount: Money) {
//         match self.accounts.get_mut(&account_id) {
//             Some(account) => {
//                 // this line was rejected by the compiler because, we didn't specify what Money + Money results to.
//                 // add Add trait to the Money struct.

//                 // theres an ERROR here too.
//                 // we don't own account, we just have mutable reference to account.
//                 // so doing account.balance + amount sends account.balance (Money(i64)) to add() fn that takes by value, ownerships
//                 // so we need to first copy the balance out add amount to it
//                 // then reassign it to account.balance
//                 account.balance = account.balance + amount;
//             }
//             None => {
//                 println!("account with id : {:?} not found!", account_id);
//             }
//         }
//     }

//     // withdraw fn, need to validate that amount is not > balance available in account
//     fn withdraw(&mut self, account_id: &AccountId, amount: Money) {
//         // needed to implement PartialOrd to unlock >, <, >=, <= for Money type.
//         if amount < Money(0) {
//             println!("amount cannot be negative");
//             return;
//         } else if amount == Money(0) {
//             println!("invalid withdrawal");
//             return;
//         }

//         match self.accounts.get_mut(account_id) {
//             Some(account) => {
//                 if amount > account.balance {
//                     println!("insufficient funds for withdrawal.");
//                     return;
//                 }
//                 // ERROR : cannot move out balance (for passing to the add() fn) because account is behind a mutable reference, we don't own the account.
//                 // SOLUTION : copy the balance value out, and reassign the account.balance field after mutating it.
//                 // we can implement Copy Trait for value like balance because its simply i64.
//                 // account is a &mut Account, so we cannot move the non-Copy
//                 // account.balance field out of it.
//                 //
//                 // account.balance - amount invokes Sub::sub(), whose `self`
//                 // parameter takes ownership of the left-hand Money value.
//                 //
//                 // Making Money Copy allows the balance to be copied for the
//                 // operation, leaving the original field in place until we
//                 // assign the returned Money back to account.balance.
//                 account.balance = account.balance - amount;
//                 println!("amount : {:?} withdrawn successfully!", amount);
//             }
//             None => {
//                 println!("account not found!");
//             }
//         }
//     }

//     // function to retrieve account balance
//     // readonly so we pass immutable reference of bank.
//     fn check_balance(&self, account_id: &AccountId) {
//         match self.accounts.get(account_id) {
//             Some(account) => {
//                 // account exists.
//                 println!(
//                     "account id : {:?} has ₹{:?} as balance",
//                     account.id, account.balance
//                 );
//             }
//             None => {
//                 println!("account doesn't exists...")
//             }
//         }
//     }

//     // validate amount
//     //     ↓
//     // validate account_one exists
//     //     ↓
//     // validate account_two exists
//     //     ↓
//     // validate account_one != account_two
//     //     ↓
//     // validate sufficient funds
//     //     ↓
//     // ONLY NOW mutate both accounts
//     // this approach has atomicity problem, what if second account doesn't exists, account one is still not getting refunded.
//     // fn transfer(&mut self, account_one: &AccountId, account_two: &AccountId, amount: Money) {
//     //     if amount < Money(0) {
//     //         println!("amount cannot be negative");
//     //         return;
//     //     }else if amount == Money(0) {
//     //         println!("invalid transfer");
//     //         return;
//     //     }

//     //     match self.accounts.get_mut(account_one) {
//     //         Some(account_one) => {
//     //             if account_one.balance < amount {
//     //                 println!("insufficient funds to transfer to another account.");
//     //                 return;
//     //             }
//     //             account_one.balance = account_one.balance - amount;

//     //             match self.accounts.get_mut(account_two) {
//     //                 Some(account) => {
//     //                     account.balance = account.balance + amount;
//     //                 },
//     //                 None => {
//     //                     println!("invalid second account...");
//     //                     account_one.balance = account_one.balance + amount;
//     //                     return;
//     //                 }
//     //             }

//     //         },
//     //         None => {
//     //             println!("invalid first account...");
//     //             return;
//     //         }
//     //     }
//     // }

//     // pattern
//     fn transfer(&mut self, account_one: &AccountId, account_two: &AccountId, amount: Money) {
//         // validate first
//         // validate account_one exists
//         // validate account_two exists
//         // make sure account_one != account_two
//         // check account_one has enough money
//         // only now mutate both accounts

//         if amount <= Money(0) {
//             println!("invalid amount");
//             return;
//         }

//         match self.accounts.get(account_one) {
//             Some(account_one) => {
//                 match self.accounts.get(account_two) {
//                     Some(account_two) => {
//                         // need to check ids for similarity
//                         if account_one.id == account_two.id {
//                             println!("invalid transfer...");
//                             return;
//                         }

//                         // instead of validating again inside the mutation phase
//                         // we can validate balance here
//                         if account_one.balance < amount {
//                             println!("insufficient balance");
//                             return;
//                         }
//                     }
//                     None => {
//                         println!("account 2 doesn't exists");
//                         return;
//                     }
//                 }
//             }
//             None => {
//                 println!("account 1 doesn't exists");
//                 return;
//             }
//         }

//         // the above code could've been this simple.
//         if account_one == account_two {
//             println!("cannot transfer to the same amount");
//             return;
//         }

//         // after the initial check, we are sure that both accounts exists & are not same.
//         // so we can proceed with normal mutation.

//         match self.accounts.get_mut(account_one) {
//             Some(account) => {
//                 if account.balance < amount {
//                     println!("error insufficient balance");
//                     return;
//                 }
//                 account.balance = account.balance - amount;
//             }
//             None => {
//                 println!("account one doesn't exists?");
//                 return;
//             }
//         }

//         match self.accounts.get_mut(account_two) {
//             Some(account) => {
//                 account.balance = account.balance + amount;
//                 println!("amount deposite in account 2 successfully...");
//             }
//             None => {
//                 println!("account 2 doesn't exists!");
//                 return;
//             }
//         }

//         println!("transaction successful...");
//     }

//     // fn transfer_2(&mut self, account_one: &AccountId, account_two: &AccountId, amount: Money) {
//     //     if let [Some(acc_one), Some(acc_two)] = self.accounts.get_disjoint_mut([account_one, account_two]) {
//     //         if acc_one.id == acc_two.id {
//     //             println!("cannot transfer to same account");
//     //             return;
//     //         }

//     //         if acc_one.balance < amount {
//     //             println!("account one has insufficient balance");
//     //             return;
//     //         }

//     //         if amount <= Money(0) {
//     //             println!("invalid amount");
//     //             return;
//     //         }

//     //         // if all checks passes - perform transfer
//     //         acc_one.balance = acc_one.balance - amount;
//     //         acc_two.balance = acc_two.balance + amount;

//     //         println!("amount transfer successfully");
//     //     } else {
//     //         println!("one or both accounts don't exists");
//     //     }
//     // }

//     fn transfer_2(&mut self, account_one: &AccountId, account_two: &AccountId, amount: Money) {
//         if amount <= Money(0) {
//             println!("invalid amount");
//             return;
//         }

//         if account_one == account_two {
//             println!("cannot transfer to same account");
//             return;
//         }

//         let [Some(acc_one), Some(acc_two)] =
//             self.accounts.get_disjoint_mut([account_one, account_two])
//         else {
//             println!("one or both accounts don't exist");
//             return;
//         };

//         if acc_one.balance < amount {
//             println!("account one has insufficient balance");
//             return;
//         }

//         // -------------------------
//         // MUTATION
//         // -------------------------

//         acc_one.balance = acc_one.balance - amount;
//         acc_two.balance = acc_two.balance + amount;

//         println!("amount transferred successfully");
//     }
// }

// #[derive(Debug, PartialEq)]
// enum AccountType {
//     Savings,
//     Current,
//     Investment,
// }

// #[derive(Debug, PartialEq)]
// enum TransactionType {
//     Deposit,
//     Withdrawal,
//     TransferOut(AccountId), // what's this ?
//     TransferIn(AccountId),  // this too ?
// }

// // creating custom types
// #[derive(Debug, Eq, Hash, PartialEq, Clone)] // what does this do and why is it important ? 
// // CustomerId isn't hashable or equatable by default, we use macros to make them h-able & e-able.
// pub struct CustomerId(String);

// #[derive(Debug, Eq, Hash, PartialEq, Clone)]
// pub struct AccountId(String);
// #[derive(Debug, Eq, Hash, PartialEq)]
// pub struct TransactionId(String);

// #[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd, Copy)]
// pub struct Money(i64); // keeping i64 instead of f64 to avoid precision errors

// use std::ops::{Add, Mul, Sub};
// // implementing Add trait for Money struct
// impl Add for Money {
//     type Output = Money;
//     // this add fn take ownership of account.balance and return Money again
//     fn add(self, rhs: Money) -> Money {
//         return Money(self.0 + rhs.0);
//     }
// }

// impl Sub for Money {
//     type Output = Money;
//     fn sub(self, rhs: Money) -> Money {
//         return Money(self.0 - rhs.0);
//     }
// }

// impl Mul for Money {
//     type Output = Money;
//     fn mul(self, rhs: Money) -> Money {
//         return Money(self.0 * rhs.0);
//     }
// }

// #[derive(Debug)] // derieves Debug so it can be printed 
// struct Customer {
//     id: CustomerId,
//     name: String,
//     email: String,
//     accounts: Vec<AccountId>, // customer only holds accountId, actual object owned by bank
// }

// impl Customer {
//     fn new(id: CustomerId, name: String, email: String) -> Self {
//         // constructor for Customer.
//         Self {
//             id,
//             name,
//             email,
//             accounts: Vec::new(), // initializes new empty accounts vector.
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// struct Account {
//     id: AccountId,
//     account_type: AccountType,
//     owner_id: CustomerId,
//     balance: Money,
//     transactions: Vec<Transaction>, // transaction objects owned by accounts
// }

// impl Account {
//     // constructor for account, balance & transactions start at a known default.
//     fn new(id: AccountId, account_type: AccountType, owner_id: CustomerId) -> Self {
//         println!("creating account for customer : {:?}", owner_id);
//         Self {
//             id,
//             account_type,
//             owner_id,
//             balance: Money(0),
//             transactions: Vec::new(),
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// struct Transaction {
//     id: TransactionId,
//     amount: Money,
//     transaction_type: TransactionType,
// }

// there is something called as ownership tree that I need to understand properly
// to understand when to use immutable reference or mutable reference.
// also I need to understand how to structure a rust program / project.


// refactored the code to be modular.

// mod tells that crates exists, but don't make the code in them avaiable in main's scope.
mod bank;
mod models;

// now we can use the bank by declaring it as bank::Bank::new() <- mod::struct::method()
// or we can use "use" keyword. 

use models::{CustomerId, AccountType, Money};
use bank::Bank;

// modules's item are private by default, 
// use "pub" keyword to make it accessible across files.

fn main() {
    // fixing the old spagetti tests.
    let mut bank = Bank::new();

    bank.create_customer("yash".to_string(), "ysonalekar@gmail.com".to_string());

    bank.create_customer("anushka".to_string(), "anu@gmail.com".to_string());

    println!("customers after creation : ");
    println!("{:?}", bank.customers);

    let customer_id_1 = CustomerId("1".to_string());
    let customer_id_2 = CustomerId("2".to_string());

    match bank.get_customer(&customer_id_1) {
        Ok(customer) => {
            println!(
                "customer found {:?} | {} | {} ",
                customer.id, customer.name, customer.email
            );
        }
        Err(e) => {
            println!("ERROR : {:?}", e);
        }
    }

    match bank.get_customer(&customer_id_2) {
        Ok(customer) => {
            println!(
                "customer found {:?} | {} | {} ",
                customer.id, customer.name, customer.email
            );
        }
        Err(e) => {
            println!("ERROR : {:?}", e);
        }
    }

    // update customer
    match bank.update_customer(
        &customer_id_1,
        "yash sonalekar".to_string(),
        "yashislearning@gmail.com".to_string(),
    ) {
        Ok(()) => {
            println!("Customer Updated Successfully");
        },
        Err(e) => {
            println!("ERROR : {:?}", e);
        }
    }

    // verify update
    match bank.get_customer(&customer_id_2) {
        Ok(customer) => {
            println!(
                "customer after update : {} | {}",
                customer.name, customer.email
            );
        }
        Err(e) => {
            println!("ERROR : {:?}",e);
        }
    }

    // create accounts

    let acc_one = match bank.create_account(&customer_id_1, AccountType::Investment) {
        Ok(acc_id) => {
            println!("account 1 created : {:?}", acc_id);
            // returning created acc_id
            acc_id
        }
        Err(e) => {
            println!("ERROR : {:?}", e);
            return;
        }
    };

    let acc_two = match bank.create_account(&customer_id_2, AccountType::Savings) {
        Ok(acc_id) => {
            println!("account 2 created : {:?}", acc_id);
            acc_id
        }
        Err(e) => {
            println!("ERROR : {:?}", e);
            return;
        }
    };

    // verify accounts exists
    match bank.get_account(&acc_one) {
        Ok(acc) => {
            println!("account one exists! : {:?}", acc.id);
        }
        Err(e) => {
            println!("ERROR : {:?}", e);
            return;
        }
    }

    match bank.get_account(&acc_two) {
        Ok(acc) => {
            println!("account two exists! : {:?}", acc.id);
        }
        Err(e) => {
            println!("ERROR : account two doesn't exists : {:?}", e);
            return;
        }
    }

    // verify account owners

    match bank.get_account_owner(&acc_one) {
        Ok(owner) => {
            println!("account {:?} belongs to {}", acc_one, owner.name);
        }
        Err(e) => {
            println!("ERROR : couldn't find owner of account one : {:?}", e);
            return;
        }
    }

    match bank.get_account_owner(&acc_two) {
        Ok(owner) => {
            println!("account {:?} belongs to {}", acc_one, owner.name);
        }
        Err(e) => {
            println!("ERROR : couldn't find owner of account 2 : {:?}", e);
        }
    }

    // deposits, withdrawals, transfers & check balance
    let _ = bank.deposit(&acc_one, Money(100_000));

    let _ = bank.check_balance(&acc_one);

    let _ = bank.withdraw(&acc_one, Money(50_000));

    let _ = bank.check_balance(&acc_one);

    let _ = bank.transfer(&acc_one, &acc_two, Money(30_000));

    let _ = bank.check_balance(&acc_one);
    let _ = bank.check_balance(&acc_two);

    let _ = bank.get_transaction_history(&acc_one);
    let _ = bank.get_transaction_history(&acc_two);

    // get all accounts

    match bank.get_all_accounts(&customer_id_1) {
        Ok(accounts) => {
            for account in accounts {
                println!("{:?}", account);
            }
        }
        Err(e) => {
            println!("ERROR : customer 1 not found : {:?}", e);
        }
    }

    // close account
    match bank.close_account(&acc_one) {
        Ok(()) => {
            println!("account closed successfully");
        }
        Err(e) => {
            println!("ERROR : {:?}", e);
        }
    }

    // verify
    match bank.get_account(&acc_one) {
        Ok(account) => {
            println!("account still exists with acc. no : {:?}", account.id);
        }, 
        Err(e) => {
            println!("account closed successfully : {:?}", e);
        }
    }

    // delete customer

    match bank.delete_customer(&customer_id_1) {
        Ok(()) => {
            println!("customer deleted successfully");
        }
        Err(e) => {
            println!("ERROR : {:?}", e);
        }
    }

    match bank.get_customer(&customer_id_1) {
        Ok(customer) => {
            println!("ERROR : customer still exists : {:?}", customer);
        }
        Err(e) => {
            println!("cutomer successfully deleted : {:?}", e);
        }
    }

    println!("TESTS COMPLETED");
}
