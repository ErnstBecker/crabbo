use std::sync::Arc;

use crate::client::AccountApi;

pub struct WalletService {
    api: Arc<dyn AccountApi>,
}

impl WalletService {
    pub fn new(api: Arc<dyn AccountApi>) -> Self {
        Self { api }
    }

    pub async fn get_balance(&self, address: &str, network: &str) -> Result<String, String> {
        const TOKEN_SYMBOL: &str = "KLV";

        let account = self.api.get_account(address, network).await?;
        let balance = account.balance as f64 / 1_000_000.0;

        Ok(format!("{:.6} {}", balance, TOKEN_SYMBOL))
    }
}
