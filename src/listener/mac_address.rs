use mac_address::{get_mac_address, MacAddress};
use std::str::FromStr;

pub fn check(requires: &Vec<String>) -> bool {
    if requires.is_empty() {
        return true;
    }

    let res = get_mac_address();
    if res.is_err() {
        return false;
    }
    let actual = res.unwrap().unwrap_or(MacAddress::default());
    debug!("current mac address is {}", actual);

    requires
        .iter()
        .any(move |r| match MacAddress::from_str(&*r.as_str()) {
            Ok(adder) => actual.eq(&adder),
            Err(_) => false,
        })
}
