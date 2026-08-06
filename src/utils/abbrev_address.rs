const ABBREV_RATIO: f64 = 0.1;

pub fn abbrev_address(address: &str) -> String {
    let size = (address.len() as f64 * ABBREV_RATIO) as usize;

    if size == 0 || address.len() <= size * 2 {
        return address.to_string();
    }

    format!(
        "{}...{}",
        &address[..size],
        &address[address.len() - size..]
    )
}
