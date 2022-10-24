//! This crate allows the calculation of the IBAN of a bank account in sweden
//!
//! It relies on the clearing number and an a specified algorithm to get the [`Iban`]
//! the function [`generate_iban`] takes a [`BankAccount`] and returns the coresponding [`Iban`]
//!
use std::fmt;
use std::str::FromStr;
use crate::clearingnumber_lookup::lookup;
use crate::IbanGenerationError::InvalidClearingNumber;

mod clearingnumber_lookup;

pub struct BankAccount {
    clearing_number: u32,
    account_number: String
}
#[derive(PartialEq, Debug)]
pub struct Iban(String);

impl Iban {
    fn new<S: Into<String>>(str: S) -> Self {
        Self(str.into())
    }
}
/// Function to pass in a [`BankAccount`], and get an [`Iban`] back
///
/// Example
///```
/// # use swedish_iban_tool::{BankAccount, generate_iban};
/// let account_number : BankAccount = "1274 0235 305".parse().unwrap();
/// let iban = generate_iban(&account_number).unwrap();
///```
pub fn generate_iban(bank_account: &BankAccount) -> Result<Iban, IbanGenerationError> {
    let result = lookup(bank_account).ok_or(InvalidClearingNumber)?;

    let identifiers = match result.method {
        Method::One => {
            format!("{:0>17}", format!("{}{}", bank_account.clearing_number, bank_account.account_number))
        }
        Method::Two => {
            format!("{:0>17}", bank_account.account_number)
        }
        Method::Three => {
            format!("{:0>17}", format!("{}{:0>10}", bank_account.clearing_number, bank_account.account_number))
        }
    };
    let check_digits = mod97(format!("{}{}", result.iban_code, identifiers)).map_err(|_| IbanGenerationError::InvalidResultingIban)?;


    return Ok(Iban::new(format!("SE{:0>2}{}{}", check_digits, result.iban_code, identifiers)))
}

fn mod97(identifier: String) -> Result<i16, ()>{
    let string_to_check = identifier + "2814" + "00";

    let check_number = string_to_check.parse::<u128>().map_err(|_| ())?;

    Ok((98 - (check_number % 97)) as i16)
}

pub(crate) enum Method {
    One,
    Two,
    Three
}
impl fmt::Display for Iban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl BankAccount {
    fn new(s: &str) -> Result<Self, ()> {
        let normalized = normalize_bban(s);
        return if s.contains(',') {
            let mut split_view = normalized.split(',');
            let fist_part = split_view.next().expect("Should allways return one element");
            let second_part = split_view.next().expect("Allways has a , at this point");

            Ok(BankAccount {
                clearing_number: fist_part.parse().map_err(|_| ())?,
                account_number: second_part.to_string()
            })
        } else {
            if normalized.len() <= 4 {
                return Err(());
            }
            let split_view = normalized.split_at(4);
            let fist_part = split_view.0;
            let second_part = split_view.1;
            Ok(BankAccount {
                clearing_number: fist_part.parse().map_err(|_| ())?,
                account_number: second_part.to_string()
            })
        }
    }
}
fn normalize_bban(str: &str) -> String {
    return str.chars().filter(|x| !matches!(x, ' ' | '-')).collect();
}
impl FromStr for BankAccount {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BankAccount::new(s)
    }

}
pub(crate) struct LookupResult {
    pub iban_code: u16,
    pub method: Method,
}

#[derive(Debug)]
pub enum IbanGenerationError {
    InvalidClearingNumber,
    InvalidResultingIban
}

#[cfg(test)]
mod tests {
    use crate::{generate_iban, Iban};

    #[test]
    fn method_one_works() {
        let account_number = "1274 0235 305";
        assert_eq!(generate_iban(&account_number.parse().unwrap()).unwrap(), Iban::new("SE4112000000012740235305"));
    }
    #[test]
    fn method_two_works() {
        let account_number = "6114 5171 82351";
        assert_eq!(generate_iban(&account_number.parse().unwrap()).unwrap(), Iban::new("SE5860000000000517182351"));
    }
    #[test]
    fn method_three_works() {
        let account_number = "8327-9, 014 725 892-5";
        assert_eq!(generate_iban(&account_number.parse().unwrap()).unwrap(), Iban::new("SE5580000832790147258925"));
    }
}
