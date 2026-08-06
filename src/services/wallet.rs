use crate::domain::AccountError;
use crate::ports::AccountApi;

pub struct WalletService {
    api: Box<dyn AccountApi>,
}

impl WalletService {
    pub fn new(api: Box<dyn AccountApi>) -> Self {
        Self { api }
    }

    pub async fn get_balance(&self, address: &str, network: &str) -> Result<f64, AccountError> {
        const TOKEN_PRECISION: f64 = 1_000_000.0;

        let account = self.api.get_account(address, network).await?;

        Ok(account.balance as f64 / TOKEN_PRECISION)
    }
}
