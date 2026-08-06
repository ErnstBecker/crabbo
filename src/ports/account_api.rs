use async_trait::async_trait;

use crate::domain::{Account, AccountError};

#[async_trait]
pub trait AccountApi: Send + Sync {
    async fn get_account(&self, address: &str, network: &str) -> Result<Account, AccountError>;
}
