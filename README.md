# swedish_iban_tool
A rust library for dealing with swedish ibans
* [crates.io](https://crates.io/crates/swedish_iban_tool)
* [docs.rs](https://docs.rs/swedish_iban_tool/0.0.1/swedish_iban_tool/)

## drawing 

![iban tool](https://user-images.githubusercontent.com/7715996/217060861-6283f518-f826-4087-b4c7-a7950ff0581d.png)


## Usage

Compile time verified examples and documentation can be found here: [docs.rs](https://docs.rs/swedish_iban_tool/0.0.1/swedish_iban_tool/)
```rust
let account_number : BankAccount = "1274 0235 305".parse().unwrap();
let iban = generate_iban(&account_number).unwrap();
```
