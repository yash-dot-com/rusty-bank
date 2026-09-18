#[derive(Debug, PartialEq)]
pub enum AccountType {
    Savings,
    Current,
    Investment,
}

#[derive(Debug, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    TransferOut(AccountId, Money), // what's this ?
    TransferIn(AccountId, Money),  // this too ?
}

// creating custom types
#[derive(Debug, Eq, Hash, PartialEq, Clone)] // what does this do and why is it important ? 
// CustomerId isn't hashable or equatable by default, we use macros to make them h-able & e-able.
pub struct CustomerId(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct AccountId(pub String);
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TransactionId(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd, Copy)]
pub struct Money(pub i64); // keeping i64 instead of f64 to avoid precision errors

use std::ops::{Add, Mul, Sub};
// implementing Add trait for Money struct
impl Add for Money {
    type Output = Money;
    // this add fn take ownership of account.balance and return Money again
    fn add(self, rhs: Money) -> Money {
        return Money(self.0 + rhs.0);
    }
}

impl Sub for Money {
    type Output = Money;
    fn sub(self, rhs: Money) -> Money {
        return Money(self.0 - rhs.0);
    }
}

impl Mul for Money {
    type Output = Money;
    fn mul(self, rhs: Money) -> Money {
        return Money(self.0 * rhs.0);
    }
}

#[derive(Debug)] // derieves Debug so it can be printed 
pub struct Customer {
    pub id: CustomerId,
    pub name: String,
    pub email: String,
    pub accounts: Vec<AccountId>, // customer only holds accountId, actual object owned by bank
}

impl Customer {
    pub fn new(id: CustomerId, name: String, email: String) -> Self {
        // constructor for Customer.
        Self {
            id,
            name,
            email,
            accounts: Vec::new(), // initializes new empty accounts vector.
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Account {
    pub id: AccountId,
    pub account_type: AccountType,
    pub owner_id: CustomerId,
    pub balance: Money,
    pub transactions: Vec<TransactionType>, // transaction objects owned by accounts
}

impl Account {
    // constructor for account, balance & transactions start at a known default.
    pub fn new(id: AccountId, account_type: AccountType, owner_id: CustomerId) -> Self {
        println!("creating account for customer : {:?}", owner_id);
        Self {
            id,
            account_type,
            owner_id,
            balance: Money(0),
            transactions: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub id: TransactionId,
    pub amount: Money,
    pub transaction_type: TransactionType,
}