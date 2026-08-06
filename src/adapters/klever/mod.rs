mod dto;
mod mapper;

use async_trait::async_trait;
use reqwest::Client;

use dto::AddressResponse;

use crate::domain::{Account, AccountError};
use crate::ports::AccountApi;

enum KleverHost {
    Node,
    #[allow(dead_code)]
    Api,
}

fn klever_url(host: KleverHost, network: &str, path: &str) -> String {
    let subdomain = match host {
        KleverHost::Node => "node",
        KleverHost::Api => "api",
    };

    format!("https://{}.{}.klever.org{}", subdomain, network, path)
}

#[derive(Clone, Default)]
pub struct KleverClient {
    client: Client,
}

impl KleverClient {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl AccountApi for KleverClient {
    async fn get_account(&self, address: &str, network: &str) -> Result<Account, AccountError> {
        let url = klever_url(KleverHost::Node, network, &format!("/address/{}", address));

        let res = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AccountError::RequestFailed(format!("Request failed: {}", e)))?;

        if !res.status().is_success() {
            return Err(AccountError::NotFound);
        }

        let body = res
            .json::<AddressResponse>()
            .await
            .map_err(|e| AccountError::RequestFailed(format!("Failed to parse response: {}", e)))?;

        Ok(mapper::to_domain_account(body.data.account))
    }
}
