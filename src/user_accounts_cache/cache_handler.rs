use std::collections::hash_map::Entry;

use crate::{transaction_event_handler::transaction_event_handler::TxEvent, user_accounts_cache::{types::AccountStore, user_accounts::UserAccountDetails}};

pub struct CacheHandler {
    user_accounts_map: AccountStore,
}

impl CacheHandler {

    pub(crate) fn new() -> Self {
        Self {
            user_accounts_map: AccountStore::new(),
        }
    }

    /// TODO: Change internal handling of errors
    /// TODO: create custom anyhow errors
    /// TODO: Create error dumping functionality
    pub(crate) fn handle_account_update(
    &mut self,
    tx_event: &TxEvent,
) -> anyhow::Result<()> {
    // Cannot do anything if account is null and the tx type is not a deposit, so handle
    let client_account_entry = self.user_accounts_map.entry(tx_event.get_client_id());
    match tx_event.get_tx_type().as_str() {
        "deposit" => {
            if let Some(valid_tx_amount) = tx_event.get_tx_amount() {
                match client_account_entry {
                    Entry::Occupied(mut occupied_entry) => {
                        occupied_entry
                            .get_mut()
                            .deposit_to_account(&valid_tx_amount);
                        Ok(())
                    }
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(UserAccountDetails::new(&valid_tx_amount));
                        Ok(())
                    }
                }
            } else {
                return Err(anyhow::format_err!(
                    "Cannot perform operation due to empty value, this should not be happening"
                ));
            }
        }
        "withdrawal" => {
            if let Entry::Occupied(mut occupied_entry) = client_account_entry {
                if let Some(valid_tx_amount) = tx_event.get_tx_amount() {
                    match occupied_entry
                        .get_mut()
                        .withdraw_from_account(&valid_tx_amount)
                    {
                        Ok(_) => Ok(()),
                        Err(withdraw_error) => Err(withdraw_error),
                    }
                } else {
                    return Err(anyhow::format_err!(
                        "Cannot perform operation due to empty value, this should not be happening"
                    ));
                }
            } else {
                Err(anyhow::format_err!(
                    "Account doesn't exist cannot withdraw from an un open account"
                ))
            }
        }
        _ => Err(anyhow::format_err!("Unhandled event")),
    }
}
}

impl std::fmt::Display for CacheHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "client,available,held,total,locked")?;
        for (client_id, account) in &self.user_accounts_map {
            writeln!(
                f,
                "{},{},{},{},{}",
                client_id,
                account.available,
                account.held,
                account.total,
                account._locked
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{transaction_event_handler::transaction_event_handler::TxEvent, CacheHandler};


    // All test Scenarios for these unit tests can be found here:
    // docs/bdd-scenarios/deposits-and-withdrawals.feature
    #[test]
    fn no_current_account_entry_deposit() {
        let mut cache_handler = CacheHandler::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event);
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details._locked, false);
        assert_eq!(account_details.held, 0.0);
    }

    #[test]
    fn no_current_entry_exists_withdraw() {
        let mut cache_handler = CacheHandler::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "withdraw".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event);
        assert!(result_under_test.is_err());
        assert_eq!(
            result_under_test.unwrap_err().to_string(),
            "Account doesn't exist cannot withdraw from an un open account"
        );

        assert_eq!(cache_handler.user_accounts_map.len(), 0);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_none());
    }

    #[test]
    fn simple_deposit_success() {
        let mut cache_handler = CacheHandler::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event);
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details._locked, false);
        assert_eq!(account_details.held, 0.0);

        let test_deposit_tx_event_two = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 2,
            amount: Some(5.0),
        };

        let result_under_test = cache_handler.handle_account_update(
            &test_deposit_tx_event_two,
        );
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 6.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details._locked, false);
        assert_eq!(account_details.held, 0.0);
    }

    #[test]
    fn simple_withdrawal_success() {
        let mut cache_handler = CacheHandler::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event);
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details._locked, false);
        assert_eq!(account_details.held, 0.0);

        let test_withdraw_tx_event_two = TxEvent {
            tx_type: "withdraw".to_string(),
            client: 1,
            tx: 2,
            amount: Some(0.5),
        };

        let result_under_test = cache_handler.handle_account_update(
            &test_withdraw_tx_event_two
        );
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 0.5);
        assert_eq!(account_details.total, 0.5);
        assert_eq!(account_details._locked, false);
        assert_eq!(account_details.held, 0.0);
    }

    #[test]
    fn simple_withdrawal_insufficient_funds() {
        let mut cache_handler = CacheHandler::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event);
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details._locked, false);
        assert_eq!(account_details.held, 0.0);

        let test_withdraw_tx_event_two = TxEvent {
            tx_type: "withdraw".to_string(),
            client: 1,
            tx: 2,
            amount: Some(2.0),
        };

        let result_under_test = cache_handler.handle_account_update(
            &test_withdraw_tx_event_two,
        );
        assert_eq!(
            result_under_test.unwrap_err().to_string(),
            "Insufficient funds to perform withdrawal"
        );

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details._locked, false);
        assert_eq!(account_details.held, 0.0);
    }
}

