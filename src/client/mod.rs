pub mod klever;

use async_trait::async_trait;
use klever::Account;

#[async_trait]
pub trait AccountApi: Send + Sync {
    async fn get_account(&self, address: &str, network: &str) -> Result<Account, String>;
}
