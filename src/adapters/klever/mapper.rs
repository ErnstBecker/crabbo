use super::dto::KleverAccount;
use crate::domain::Account;

pub(super) fn to_domain_account(dto: KleverAccount) -> Account {
    Account {
        balance: dto.balance,
    }
}
