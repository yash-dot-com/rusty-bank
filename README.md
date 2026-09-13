# RustBank

This project is a small banking example written in Rust to practice ownership, borrowing, and data modeling in a realistic domain.

## What is in this project

From the code in [src/main.rs](src/main.rs), the project includes:

- `Bank` with `customers` and `accounts` stored in `HashMap`
- `Customer` with an ID, name, email, and a list of account IDs
- `Account` with an ID, account type, owner ID, balance, and transactions
- `Money` as a custom value type around `i64`
- methods for:
  - creating a customer
  - getting a customer
  - updating a customer
  - deleting a customer
  - creating an account
  - getting an account
  - getting the account owner
  - listing customer accounts
  - closing an account
  - depositing money
  - withdrawing money
  - checking balance
  - transferring between accounts

## Learning notes from the code

### Ownership and move semantics

The code specifically shows the issue of moving part of a struct:

```rust
let customer_id = customer.id.clone();
self.customers.insert(customer_id, customer);
```

This is used because `customer.id` cannot simply be moved out while the whole `customer` is still being used. The comment in the code explains that this is a partial move problem and why cloning the ID is the safe fix.

### Borrowing vs ownership

The project uses both read-only and mutable access patterns:

```rust
fn get_customer(&self, customerid: &CustomerId) -> Option<&Customer>
fn update_customer(&mut self, customer_id: &CustomerId, name: String, email: String)
```

This demonstrates the difference between:

- `&self` for read access
- `&mut self` for modifying data
- passing `&CustomerId` so the ID is borrowed instead of moved

### Why `Clone` is needed

The custom ID types are derived with `Clone`:

```rust
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct CustomerId(String);
```

This is needed because the IDs are stored in hash maps and sometimes reused across data structures without taking ownership of the original value.

### Why `Copy` is needed for `Money`

The project defines:

```rust
#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd, Copy)]
pub struct Money(i64);
```

This matters because the code does arithmetic on borrowed values such as:

```rust
account.balance = account.balance - amount;
```

The comments explain that `Money` must be `Copy` so the value can be used temporarily without moving it out of the borrowed `account`.

### HashMap usage and mutation

The bank stores its state in hash maps:

```rust
struct Bank {
    customers: HashMap<CustomerId, Customer>,
    accounts: HashMap<AccountId, Account>,
}
```

The code also shows the important difference between:

```rust
self.customers.get(customer_id)
```

and:

```rust
self.customers.get_mut(customer_id)
```

The first gives an immutable reference; the second is required when changing data.

## Common mistakes explained in the code comments

The source file contains direct notes about the main Rust mistakes the project was learning from.

### 1. Moving a field out of a struct

The code comments explain that this is wrong:

```rust
customer.id
```

when the whole `customer` is still needed. The fix in this project is to clone the ID before inserting the customer into the map.

### 2. Using immutable access for mutation

The file explicitly notes that `&self` is wrong for operations that modify state, and that `&mut self` is required for update-like methods.

### 3. Borrowed values and arithmetic

The comments show that expressions like `account.balance + amount` or `account.balance - amount` fail when the balance is inside a borrowed mutable object. The solution is to make `Money` `Copy` and assign the result back to the field.

### 4. Partial ownership and insertion

The file discusses a partial move issue when an ID is moved before the complete object is inserted into the map.

## Project behavior from the code

The project is built around the following rules, which are also written in the comments at the top of the file:

- cannot withdraw more than the balance
- amount must be positive
- customer must exist before creating an account
- transfer should not be done to a non-existent account
- transfer to self is not allowed
- account IDs and customer IDs must be unique
- closed accounts cannot transact

## Summary

This project is mainly a Rust learning exercise built around a banking domain. The key learning in the code is centered on:

- ownership
- moving vs borrowing
- cloning IDs
- using `Copy` for small value types
- mutable access with `&mut self`
- storing data in `HashMap`
- safe design for account/customer relationships

The README reflects only the concepts and project details that are directly visible in the current Rust implementation and comments.
account.balance = account.balance + amount;

### Mistake 6: trying to borrow while mutably borrowing elsewhere

The design of `Bank` intentionally balances borrowing carefully because the bank owns both `customers` and `accounts` hash maps. Methods often do a lookup and mutate only the selected item, rather than trying to mutate the whole bank in one step.

### Mistake 7: removing account data without updating the customer list

When closing an account, the implementation must do both of these correctly:

1. remove the account from `Bank.accounts`
2. remove the account ID from the owning customer’s `accounts` vector

This is the “consistent transfer” style of data cleanup: the bank and customer data must stay aligned.

## Ownership model in plain language

The project is a practical teaching example for understanding Rust ownership rules:

- values can be owned by only one place at a time
- references allow temporary access without ownership transfer
- `clone` creates a new value when needed
- `Copy` can be used for small value types like `Money`
- `HashMap` keys and values must support equality and hashing
- lifetimes and borrowing are enforced by the compiler, not by runtime checks

This is exactly why the project uses custom types like:

```rust
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct CustomerId(String);

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct AccountId(String);
```

Without these derives, the IDs could not be used as hash map keys.

## Account operations implemented

The bank supports:

- create customer
- get customer by ID
- update customer
- delete customer
- create account for a customer
- get account details
- get owner of account
- list all accounts for a customer
- close an account
- deposit money
- withdraw money
- check balance
- transfer money between accounts

## Error handling philosophy

The project is intentionally designed around safe runtime checks rather than unsafe panic-based behavior. The code prints useful messages when something is invalid, such as:

- insufficient funds
- account not found
- customer not found
- invalid transfer amounts
- invalid withdrawal amount

This teaches a Rust pattern of handling invalid states early and returning control safely.

## Notes on design

### Why the bank owns all accounts

This is a good design because the bank is the central authority. It tracks:

- which accounts exist
- who owns each account
- current balances
- all account-level data

Customers only keep references to account IDs, rather than owning their full account objects.

### Why `AccountId` and `CustomerId` are custom wrappers

Custom ID wrappers provide:

- semantic clarity
- stronger type safety
- unique meaning for each identifier type
- compatibility with `HashMap` keys

## Example flow for a user session

A typical flow could look like this:

1. create a bank
2. create a customer
3. create an account for that customer
4. deposit funds
5. withdraw funds
6. check balance
7. transfer to another account
8. close account when necessary
9. delete customer if required

## Summary

This project is not just a banking app; it is a Rust learning project built around the real compiler rules that shape safe code.

The most important lessons were:

- moving values changes ownership
- borrowing lets you access values without taking ownership
- `Clone` and `Copy` are essential when managing IDs and money values
- `HashMap` is powerful but requires consistent key types
- the compiler enforces correctness early and consistently
- design clarity matters as much as syntax

The project is a strong example of learning Rust by building a realistic problem domain rather than only reading isolated syntax.

## Final takeaways

Rust is strict, but the strictness is the strength of the language. The project shows that once ownership and borrowing are understood, code becomes safer and easier to reason about. The bank example is a practical way to internalize those rules.

This repository demonstrates that strong domain modeling and correct memory ownership are not separate ideas—they are part of the same design discipline in Rust.
