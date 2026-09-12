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

    //  refactoring create_account() fn to return AccountId. 
    // returns &AccountId because I don't want to move accountid out of the account object.
    fn create_account(&mut self, customer_id: &CustomerId, account_type: AccountType) -> Option<AccountId> {
        match self.customers.get_mut(customer_id) {
            Some(customer) => {
                let account_id = AccountId("123".to_string());

                let account = Account::new(
                    account_id.clone(),
                    account_type,
                    customer.id.clone(),
                );

                customer.accounts.push(account_id.clone());
                // cloning account_id before accounts hashmap owns the account_id 
                let returned_id = account_id.clone();

                self.accounts.insert(account_id, account);
                Some(returned_id)

                // ownership flow
                //                     ┌── clone → Account.id
                //                     │
                // account_id ─────────┼── clone → Customer.accounts
                //                     │
                //                     ├── clone → returned_id
                //                     │
                //                     └── move → Bank.accounts HashMap key

            },
            None => {
                println!("customer doesn't exists, account cannot be created.");
                None
            },
        }
    }

    fn get_account(&self, account_id: &AccountId) -> Option<&Account> {
        // check if account exists, return &Account 
        // else return None 
        // match self.accounts.get(account_id) {
        //     Some(account) => Some(account), 
        //     None => None,
        // }

        // also the hashmap returns Option<&Account> so we can just put 
        self.accounts.get(account_id)
    }

    fn get_account_owner(&self, account_id: &AccountId) -> Option<&Customer> {
        // check if account exists 
        // if yes then return its customer 
        // if no then return none.
        match self.accounts.get(account_id) {
            Some(account) => {
                match self.customers.get(&account.owner_id) {
                    Some(owner) => Some(owner),
                    None => None,
                }
            },
            None => None,
        }
    }

    // fn to list customer's all accounts must return Option<Vec<Account>>
    fn get_all_accounts(&self, customer_id: &CustomerId) -> Option<Vec<&Account>> {
        // bank owns account 
        // get_all_accounts() -> borrows accounts -> Vec<&Account> 
        // cli iterates and prints 
        match self.customers.get(customer_id) {
            Some(customer) => {
                println!("customer found: {}", customer.name);
                let mut accounts = Vec::new();

                // if no account id we return empty vector

                for account_id in &customer.accounts {
                    match self.accounts.get(account_id){
                        Some(account) => accounts.push(account),
                        None => {}
                    }
                }
                // return Some(accounts) after constructing the vector
                Some(accounts)
            },
            None => None
        }
    }

}

#[derive(Debug)]
enum AccountType {
    Savings,
    Current,
    Investment,
}

#[derive(Debug)]
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

#[derive(Debug, Eq, Hash, PartialEq, Clone)] 
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

#[derive(Debug)]
struct Account {
    id: AccountId,
    account_type: AccountType,
    owner_id: CustomerId,
    balance: Money, 
    transactions: Vec<Transaction>, // transaction objects owned by accounts 
}

impl Account {
    // constructor for account, balance & transactions start at a known default.
    fn new(
        id: AccountId,
        account_type: AccountType,
        owner_id: CustomerId
    ) -> Self {
        println!("creating account for customer : {:?}", owner_id);
        Self {
            id, account_type, owner_id, balance: Money(0), transactions: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Transaction {
    id: TransactionId,
    amount: Money, 
    transaction_type: TransactionType,
}


// there is something called as ownership tree that I need to understand properly 
// to understand when to use immutable reference or mutable reference. 
// also I need to understand how to structure a rust program / project. 
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

    // create account for customer 
    // Account -> I own the account 
    // &Account -> I borrow the account 
    // T -> the thing itself 
    // &T -> temporary borrowed reference to that thing.
    let account_id_1 = bank.create_account(&customerid, AccountType::Savings);
    match account_id_1 {
        Some(account_id) => {
            println!("account created : {:?}", account_id);

            match bank.get_account(&account_id) {
                Some(account) => {println!("account found : {:?}", account.id)},
                None => println!("account not found"),
            }

            match bank.get_account_owner(&account_id) {
                Some(owner) => {println!("owner found : {:?}", owner.name)},
                None => {println!("owner not found")},
            }
        },

        None => println!("account creation failed"),
    }

    let customer = bank.get_account_owner(&AccountId("123".to_string()));
    match customer{
        Some(customer) => println!("customer name owning account id : {} is {}", "123" ,customer.name),
        None => println!("customer doesn't exists"),
    }

    let all_accounts = bank.get_all_accounts(&CustomerId("123".to_string()));
    match all_accounts {
        Some(accounts) => {
            for account in accounts {
                println!("{:?}", account);
            }
        }, 
        None => println!("No accounts found!"),
    }
    
    bank.delete_customer(&customerid);
    // this not required because HashMap.remove() throws data, so we can own it since its not owned by the hashmap anymore and consume, discard it.
    // match bank.get_customer(&customerid){
    //     Some(customer) => println!("customer not deleted! {}", customer.name),
    //     None => println!("customer not found!"),
    // }
}
