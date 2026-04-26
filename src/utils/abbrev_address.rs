pub fn abbrev_address(address: &str) -> String {
    if address.len() > 10 {
        format!("{}...{}", &address[..5], &address[address.len() - 5..])
    } else {
        address.to_string()
    }
}
