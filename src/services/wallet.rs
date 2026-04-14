use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct AddressResponse {
    data: AddressData,
}

#[derive(Deserialize)]
struct AddressData {
    account: Account,
}

#[derive(Deserialize)]
struct Account {
    balance: u64,
}

pub struct WalletService {
    client: Client,
}

impl WalletService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_balance(&self, address: &str, _network: &str) -> Result<String, String> {
        let klever_api: String = format!("https://api.{:}.klever.finance/v1.0", _network);
        const TOKEN_SYMBOL: &str = "KLV";

        let url = format!("{}/address/{}", klever_api, address);

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

        let klv = body.data.account.balance as f64 / 1_000_000.0;

        Ok(format!("{:.6} {:}", klv, TOKEN_SYMBOL))
    }
}
