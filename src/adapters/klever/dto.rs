use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct AddressResponse {
    pub(super) data: AddressData,
}

#[derive(Deserialize)]
pub(super) struct AddressData {
    pub(super) account: KleverAccount,
}

/// Klever API's account representation. Chain-specific shape (PascalCase
/// fields) — must be mapped to `domain::Account` before leaving this adapter.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(super) struct KleverAccount {
    pub(super) balance: u64,
}
