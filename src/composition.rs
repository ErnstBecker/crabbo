use crate::adapters::klever::KleverClient;
use crate::commands::CommandManager;
use crate::services::wallet::WalletService;

/// Wires adapters into services and hands them to the command manager.
pub fn bootstrap() -> CommandManager {
    let klever_client = KleverClient::new();
    let wallet_service = WalletService::new(Box::new(klever_client));

    CommandManager::build(wallet_service)
}
