// bank --owns-- customer --owns-- accounts (saving, current, investments) --owns-- transactions
use std::{collections::HashMap};
use crate::models::{CustomerId, Customer, Account, AccountId, AccountType, Money};

// mutable reference of bank will allow mutable access to all the variables owned by it.
pub struct Bank {
    pub customers: HashMap<CustomerId, Customer>,
    pub accounts: HashMap<AccountId, Account>,
    next_customer_id: u64,
    next_account_id: u64,
}

enum BankError {
    InsufficientFunds,
    CustomerDoesNotExists,
    CustomerHasActiveAccounts,
    FailedToCreateAccount,
    AccountNotFound,
}

// lets implement Bank methods
// bank must be able to create customers

impl Bank {
    // revised.
    // constructor for bank object
    pub fn new() -> Self {
        // create an empty customer & account hashmap when bank object is created.
        println!("bank created successfully!");
        return Bank {
            customers: HashMap::new(),
            accounts: HashMap::new(),
            next_customer_id: 1,
            next_account_id: 1,
        };
    }

    // ERROR : failed to create customer
    pub fn create_customer(&mut self, name: String, email: String) -> Option<Customer> {
        // responsible only for creation, the data input should be handled by the main function.
        // let id: CustomerId = CustomerId("123".to_string()); // rust doesn't implicitly converts datatypes for us, so manually converted &str (reference of string baked into main program) to String (stored on heap during runtime) then into customerId.
        let id = CustomerId(self.next_customer_id.to_string());
        self.next_customer_id += 1;

        // cloned the id value here to be able to use later.
        let customer = Customer::new(id.clone(), name, email);

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
        // let customer_id = customer.id.clone();

        // cloned the id previously to use here.
        self.customers.insert(id, customer)
    }

    // need only read access
    // ERROR : customer doesn't exists 
    pub fn get_customer(&self, customerid: &CustomerId) -> Result<&Customer, BankError> {
        // functions takes ownership of value if not passed with reference, here we take CustomerId as reference because we only want to look it up
        // refactor, we just want to return the reference to the found customer.
        // cli will decide what to do with it.
        match self.customers.get(customerid) {
            Some(customer) => Ok(&customer),
            None => Err(BankError::CustomerDoesNotExists)
        }
    }

    // revised.
    // &self <- WRONG, we need mutable reference &mut self.
    // read  → &self + get()
    // write → &mut self + get_mut()
    pub fn update_customer(&mut self, customer_id: &CustomerId, name: String, email: String) -> Result<(), BankError> {
        // we need mutable reference to the customer object to actually change its content.
        // let mut customer = self.customers.get(customer_id); // <- WRONG .get return immutable reference & we need &mut reference.
        let customer = self.customers.get_mut(customer_id);
        match customer {
            Some(customer) => {
                customer.name = name;
                customer.email = email;
                // we produce () empty tuple for fn that returns nothing. 
                Ok(())
            }

            None => {
                Err(BankError::CustomerDoesNotExists)
            }
        }
    }

    // delete customer needs reference to CustomerId
    // ERROR : either returns () or BankError when customer doesn't exists, can't delete something that doesn't exist
    pub fn delete_customer(&mut self, customer_id: &CustomerId) -> Result<(), BankError> {
        // self.customers.remove(customer_id); // .remove() returns Option<V> containing removed value if the key existed in the map.
        // notice .remove returns Option<V> not Option<&V> that means its throwing away the data, removing it from hashmap and giving us the ownership of removed data

        match self.get_customer(customer_id) {
            Ok(customer) => {
                if !customer.accounts.is_empty() {
                    Err(BankError::CustomerHasActiveAccounts)
                } else {
                    self.customers.remove(customer_id);
                    Ok(())
                }
            }, 

            // get customer already gives an error return it only
            Err(error) => {
                Err(error)
            }
        }

        // the above code can be written as 
        // let customer = self.get_customer(customer_id)?
        // if !customer.accounts.is_empty() {
        //     Err(BankError::CustomerHasActiveAccounts)
        // } else {
        //     self.customers.remove(&customer.id);
        //     Ok(())
        // }

        // match self.customers.remove(customer_id) {
        //     Some(customer) => {
        //         // while removing a customer, we also remove all of its accounts from bank.
        //         // for accounts in customer.accounts {
        //         //     self.accounts.remove(&accounts);
        //         // }
        //         // design change, if a customer has active accounts, can't delete it. first withdraw balance, then delete accounts then delete customer
        //         if !customer.accounts.is_empty() {
        //             Err(BankError::CustomerHasActiveAccounts)
        //         } else {
        //             Ok(())
        //         }
        //     }
        //     None => Err(BankError::CustomerDoesNotExists)
        // }
    }

    // customer CRUD done.
    // lets build account CRUD
    // fn create_account(&mut self, customer_id: &CustomerId, account_type: AccountType, initial_balance: Money = 0, ) {
    //     // check if customer exists
    //     // generate new account id
    //     // create new account
    //     // store account in self.hashmap & customer.accounts vector
    //     // MAJOR OWNERSHIP REFACTOR
    //     // bank owns the accountid field for hashmap
    //     // customer only keeps the reference because bank is the source of truth.
    //     match self.customers.get_mut(customer_id) {
    //         Some(customer) => {
    //             let account_id = AccountId("123".to_string());
    //             let customer_id = customer.id.clone(); // cloning customer id to store in hashmap without taking ownership from customer object.
    //             let account = Account::new(account_id.clone(), account_type, customer_id);

    //             customer.accounts.push(account_id.clone()); // account_id already moved.
    //             self.accounts.insert(account_id, account); // again account_id already moved.
    //         }
    //         None => println!("Customer doesn't exists, Account cannot be created.")
    //     }
    // }

    // refactoring create_account() fn to return AccountId.
    // returns &AccountId because I don't want to move accountid out of the account object.
    pub fn create_account(
        &mut self,
        customer_id: &CustomerId,
        account_type: AccountType,
    ) -> Result<AccountId, BankError> {
        match self.customers.get_mut(customer_id) {
            Some(customer) => {
                // let account_id = AccountId("123".to_string());
                let account_id = AccountId(self.next_account_id.to_string());
                self.next_account_id += 1;

                let account = Account::new(account_id.clone(), account_type, customer.id.clone());

                customer.accounts.push(account_id.clone());
                // cloning account_id before accounts hashmap owns the account_id
                let returned_id = account_id.clone();

                self.accounts.insert(account_id, account);
                Ok(returned_id)

                // ownership flow
                //                     ┌── clone → Account.id
                //                     │
                // account_id ─────────┼── clone → Customer.accounts
                //                     │
                //                     ├── clone → returned_id
                //                     │
                //                     └── move → Bank.accounts HashMap key
            }
            None => {
                Err(BankError::CustomerDoesNotExists)
            }
        }
    }

    pub fn get_account(&self, account_id: &AccountId) -> Result<&Account, BankError> {
        // check if account exists, return &Account
        // else return None
        // match self.accounts.get(account_id) {
        //     Some(account) => Some(account),
        //     None => None,
        // }

        // also the hashmap returns Option<&Account> so we can just put
        match self.accounts.get(account_id) {
            Some(account) => {
                Ok(account)
            },
            None => Err(BankError::AccountNotFound)
        }

    }

    // either returns reference to customer or BankError saying customer doesn't exists.
    pub fn get_account_owner(&self, account_id: &AccountId) -> Result<&Customer, BankError> {
        // check if account exists
        // if yes then return its customer
        // if no then return none.
        match self.accounts.get(account_id) {
            Some(account) => match self.customers.get(&account.owner_id) {
                Some(owner) => Ok(owner),
                None => Err(BankError::CustomerDoesNotExists),
            },
            None => Err(BankError::AccountNotFound),
        }
    }

    // fn to list customer's all accounts must return Option<Vec<Account>>
    pub fn get_all_accounts(&self, customer_id: &CustomerId) -> Result<Vec<&Account>, BankError> {
        // bank owns account
        // get_all_accounts() -> borrows accounts -> Vec<&Account>
        // cli iterates and prints

        match self.get_customer(customer_id) {
            Ok(customer) => {
                println!("customer found: {}", customer.name);
                let mut accounts = Vec::new();

                // if no account id we return empty vector

                for account_id in &customer.accounts {
                    // alternate way : ?
                    // let account = self.get_account(account_id)?;
                    // accounts.push(account);

                    match self.get_account(account_id) {
                        Ok(account) => accounts.push(account),
                        Err(e) => return Err(e) // i am inside for loop so need to explicitly return Error.
                    }
                }
                // return Some(accounts) after constructing the vector
                Ok(accounts)
            },
            Err(error) => Err(error)
        }
    }

    pub fn close_account(&mut self, account_id: &AccountId) -> Result<(), BankError> {
        // first lets check if account exists in the bank
        // if yes then lets remove it from bank
        // then we will proceed to remove from it from owner's account id vector.
        //     match self.accounts.get(account_id) {
        //         Some(account) => {
        //             // if the account exists,
        //             let owner_id = account.owner_id.clone();
        //             let owner = self.customers.get_mut(&owner_id);
        //             match owner {
        //                 Some(owner) => {
        //                     owner.accounts.retain(|id| id != account_id);
        //                 },
        //                 None => {
        //                     println!("account has no owner!")
        //                 }
        //             }

        //             match self.accounts.remove(account_id) {
        //                 Some(_) => {
        //                     println!("account closed successfully!");
        //                 },
        //                 None => {
        //                     println!("could not close account");
        //                 }
        //             }

        //         },
        //         None => {
        //             println!("account doesn't exists!");
        //         }
        // }

        // clever code
        // remove(account_id)
        //     ↓
        // Option<Account>
        //     ↓
        // Some(account)
        //     ↓
        // account.owner_id
        //     ↓
        // find customer
        //     ↓
        // retain account_id

        // flaws in current code, the account is removed no matter if it results in error or success. 
        // we are already removing the account 

        // 1. Does account exist?
        // 2. Who owns it?
        // 3. Does owner exist?
        // 4. Only then remove account
        // 5. Remove account ID from owner
        // 6. Ok(())
        match self.accounts.remove(account_id) {
            Some(account) => {
                let owner_id = account.owner_id;
                match self.customers.get_mut(&owner_id) {
                    Some(owner) => {
                        owner.accounts.retain(|id| id != account_id);
                        println!("account closed successfully!");
                        Ok(())
                    }
                    None => {
                        println!("owner not found!");
                        Err(BankError::CustomerDoesNotExists)
                    }
                }
            }
            None => {
                println!("account not found!");
                Err(BankError::AccountNotFound)
            }
        }
        // how its working :-
        // Bank.accounts
        //     │
        //     │ remove(account_id)
        //     ↓
        // Account  ← you own it
        //     │
        //     │ move owner_id
        //     ↓
        // Customer
        //     │
        //     │ retain()
        //     ↓
        // remove AccountId from customer's Vec
    }

    // day 3 - implementing bank operations
    // fn deposit
    pub fn deposit(&mut self, account_id: &AccountId, amount: Money) {
        match self.accounts.get_mut(&account_id) {
            Some(account) => {
                // this line was rejected by the compiler because, we didn't specify what Money + Money results to.
                // add Add trait to the Money struct.

                // theres an ERROR here too.
                // we don't own account, we just have mutable reference to account.
                // so doing account.balance + amount sends account.balance (Money(i64)) to add() fn that takes by value, ownerships
                // so we need to first copy the balance out add amount to it
                // then reassign it to account.balance
                account.balance = account.balance + amount;
            }
            None => {
                println!("account with id : {:?} not found!", account_id);
            }
        }
    }

    // withdraw fn, need to validate that amount is not > balance available in account
    pub fn withdraw(&mut self, account_id: &AccountId, amount: Money) {
        // needed to implement PartialOrd to unlock >, <, >=, <= for Money type.
        if amount < Money(0) {
            println!("amount cannot be negative");
            return;
        } else if amount == Money(0) {
            println!("invalid withdrawal");
            return;
        }

        match self.accounts.get_mut(account_id) {
            Some(account) => {
                if amount > account.balance {
                    println!("insufficient funds for withdrawal.");
                    return;
                }
                // ERROR : cannot move out balance (for passing to the add() fn) because account is behind a mutable reference, we don't own the account.
                // SOLUTION : copy the balance value out, and reassign the account.balance field after mutating it.
                // we can implement Copy Trait for value like balance because its simply i64.
                // account is a &mut Account, so we cannot move the non-Copy
                // account.balance field out of it.
                //
                // account.balance - amount invokes Sub::sub(), whose `self`
                // parameter takes ownership of the left-hand Money value.
                //
                // Making Money Copy allows the balance to be copied for the
                // operation, leaving the original field in place until we
                // assign the returned Money back to account.balance.
                account.balance = account.balance - amount;
                println!("amount : {:?} withdrawn successfully!", amount);
            }
            None => {
                println!("account not found!");
            }
        }
    }

    // function to retrieve account balance
    // readonly so we pass immutable reference of bank.
    pub fn check_balance(&self, account_id: &AccountId) {
        match self.accounts.get(account_id) {
            Some(account) => {
                // account exists.
                println!(
                    "account id : {:?} has ₹{:?} as balance",
                    account.id, account.balance
                );
            }
            None => {
                println!("account doesn't exists...")
            }
        }
    }

    // validate amount
    //     ↓
    // validate account_one exists
    //     ↓
    // validate account_two exists
    //     ↓
    // validate account_one != account_two
    //     ↓
    // validate sufficient funds
    //     ↓
    // ONLY NOW mutate both accounts
    // this approach has atomicity problem, what if second account doesn't exists, account one is still not getting refunded.
    // fn transfer(&mut self, account_one: &AccountId, account_two: &AccountId, amount: Money) {
    //     if amount < Money(0) {
    //         println!("amount cannot be negative");
    //         return;
    //     }else if amount == Money(0) {
    //         println!("invalid transfer");
    //         return;
    //     }

    //     match self.accounts.get_mut(account_one) {
    //         Some(account_one) => {
    //             if account_one.balance < amount {
    //                 println!("insufficient funds to transfer to another account.");
    //                 return;
    //             }
    //             account_one.balance = account_one.balance - amount;

    //             match self.accounts.get_mut(account_two) {
    //                 Some(account) => {
    //                     account.balance = account.balance + amount;
    //                 },
    //                 None => {
    //                     println!("invalid second account...");
    //                     account_one.balance = account_one.balance + amount;
    //                     return;
    //                 }
    //             }

    //         },
    //         None => {
    //             println!("invalid first account...");
    //             return;
    //         }
    //     }
    // }

    // pattern
    pub fn transfer(&mut self, account_one: &AccountId, account_two: &AccountId, amount: Money) {
        // validate first
        // validate account_one exists
        // validate account_two exists
        // make sure account_one != account_two
        // check account_one has enough money
        // only now mutate both accounts

        if amount <= Money(0) {
            println!("invalid amount");
            return;
        }

        match self.accounts.get(account_one) {
            Some(account_one) => {
                match self.accounts.get(account_two) {
                    Some(account_two) => {
                        // need to check ids for similarity
                        if account_one.id == account_two.id {
                            println!("invalid transfer...");
                            return;
                        }

                        // instead of validating again inside the mutation phase
                        // we can validate balance here
                        if account_one.balance < amount {
                            println!("insufficient balance");
                            return;
                        }
                    }
                    None => {
                        println!("account 2 doesn't exists");
                        return;
                    }
                }
            }
            None => {
                println!("account 1 doesn't exists");
                return;
            }
        }

        // the above code could've been this simple.
        if account_one == account_two {
            println!("cannot transfer to the same amount");
            return;
        }

        // after the initial check, we are sure that both accounts exists & are not same.
        // so we can proceed with normal mutation.

        match self.accounts.get_mut(account_one) {
            Some(account) => {
                if account.balance < amount {
                    println!("error insufficient balance");
                    return;
                }
                account.balance = account.balance - amount;
            }
            None => {
                println!("account one doesn't exists?");
                return;
            }
        }

        match self.accounts.get_mut(account_two) {
            Some(account) => {
                account.balance = account.balance + amount;
                println!("amount deposite in account 2 successfully...");
            }
            None => {
                println!("account 2 doesn't exists!");
                return;
            }
        }

        println!("transaction successful...");
    }

    // fn transfer_2(&mut self, account_one: &AccountId, account_two: &AccountId, amount: Money) {
    //     if let [Some(acc_one), Some(acc_two)] = self.accounts.get_disjoint_mut([account_one, account_two]) {
    //         if acc_one.id == acc_two.id {
    //             println!("cannot transfer to same account");
    //             return;
    //         }

    //         if acc_one.balance < amount {
    //             println!("account one has insufficient balance");
    //             return;
    //         }

    //         if amount <= Money(0) {
    //             println!("invalid amount");
    //             return;
    //         }

    //         // if all checks passes - perform transfer
    //         acc_one.balance = acc_one.balance - amount;
    //         acc_two.balance = acc_two.balance + amount;

    //         println!("amount transfer successfully");
    //     } else {
    //         println!("one or both accounts don't exists");
    //     }
    // }

    pub fn transfer_2(&mut self, account_one: &AccountId, account_two: &AccountId, amount: Money) {
        if amount <= Money(0) {
            println!("invalid amount");
            return;
        }

        if account_one == account_two {
            println!("cannot transfer to same account");
            return;
        }

        let [Some(acc_one), Some(acc_two)] =
            self.accounts.get_disjoint_mut([account_one, account_two])
        else {
            println!("one or both accounts don't exist");
            return;
        };

        if acc_one.balance < amount {
            println!("account one has insufficient balance");
            return;
        }

        // -------------------------
        // MUTATION
        // -------------------------

        acc_one.balance = acc_one.balance - amount;
        acc_two.balance = acc_two.balance + amount;

        println!("amount transferred successfully");
    }
}
