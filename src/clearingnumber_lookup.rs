use crate::{BankAccount, Method};
use crate::Method::{One, Three, Two};
use crate::LookupResult;

pub(crate) fn lookup(bank_account: &BankAccount) -> Option<LookupResult> {
    //Only ever check the first four
    let number_to_check: u16 = bank_account.clearing_number.to_string().chars().take(4).collect::<String>().parse().expect("Should only ever be a number not bigger then u16");

    let result: Option<(u16, Method)> = match number_to_check {
        1100..=1199 => Some((300, One)),
        1200..=1399 => Some((120, One)),
        1400..=2099 => Some((300, One)),
        2300..=2399 => Some((230, One)),
        2400..=2499 => Some((120, One)),
        3300 => Some((300, One)),
        3000..=3399 => Some((300, One)),
        3400..=3409 => Some((902, One)),
        3782 => Some((300, One)),
        3410..=4999 => Some((300, One)),
        5000..=5999 => Some((500, One)),
        6000..=6999 => Some((600, Two)),
        7000..=7999 => Some((800, One)),
        8000..=8999 => Some((800, Three)),
        9020..=9029 => Some((902, One)),
        9040..=9049 => Some((904, One)),
        9060..=9069 => Some((902, One)),
        9070..=9079 => Some((907, One)),
        9100..=9109 => Some((910, One)),
        9120..=9124 => Some((500, One)),
        9130..=9149 => Some((500, One)),
        9150..=9169 => Some((915, One)),
        9170..=9179 => Some((917, One)),
        9190..=9199 => Some((919, One)),
        9230..=9239 => Some((923, One)),
        9250..=9259 => Some((925, One)),
        9270..=9279 => Some((927, One)),
        9300..=9349 => Some((930, One)),
        9280..=9289 => Some((928, One)),
        9390..=9399 => Some((939, One)),
        9400..=9449 => Some((940, One)),
        9460..=9469 => Some((946, One)),
        9470..=9479 => Some((947, One)),
        9500..=9549 => Some((950, Two)),
        9550..=9569 => Some((955, One)),
        9570..=9579 => Some((957, Two)),
        9580..=9589 => Some((958, One)),
        9590..=9599 => Some((959, One)),
        9630..=9639 => Some((963, One)),
        9640..=9649 => Some((964, One)),
        9650..=9659 => Some((965, One)),
        9660..=9669 => Some((966, One)),
        9670..=9679 => Some((967, One)),
        9680..=9689 => Some((968, One)),
        9700..=9709 => Some((970, One)),
        9750..=9759 => Some((975, One)),
        9780..=9789 => Some((978, One)),
        9960..=9969 => Some((950, Two)),
        _ => None
    };

    match result {
        None => {
            None
        }
        Some(x) => {
            Some(LookupResult {
                iban_code: x.0,
                method: x.1
            })
        }
    }
}

