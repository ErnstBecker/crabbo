use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use super::AccountApi;

#[derive(Deserialize)]
struct AddressResponse {
    data: AddressData,
}

#[derive(Deserialize)]
struct AddressData {
    account: Account,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Account {
    pub(crate) balance: u64,
}

pub struct KleverClient {
    client: Client,
}

impl Default for KleverClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl AccountApi for KleverClient {
    async fn get_account(&self, address: &str, network: &str) -> Result<Account, String> {
        let url = format!("https://node.{}.klever.org/address/{}", network, address);

        let res = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !res.status().is_success() {
            return Err("Address not found or invalid".to_string());
        }

        let body = res
            .json::<AddressResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(body.data.account)
    }
}
