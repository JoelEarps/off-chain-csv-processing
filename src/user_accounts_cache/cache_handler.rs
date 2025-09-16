use std::collections::hash_map::Entry;

use anyhow::Context;

use crate::{
    transaction_handler::{
        cache_handler::TransactionCache,
        types::{TransactionState, TransactionTrackerEntry, TxEvent},
    },
    user_accounts_cache::{types::AccountStore, user_accounts::UserAccountDetails},
};

pub struct CacheHandler {
    user_accounts_map: AccountStore,
    transaction_store: TransactionCache,
}

impl CacheHandler {
    pub(crate) fn new() -> Self {
        Self {
            user_accounts_map: AccountStore::new(),
            transaction_store: TransactionCache::new(),
        }
    }

    pub(crate) fn handle_account_update(&mut self, tx_event: &TxEvent) -> anyhow::Result<()> {
        let client_account_entry = self.user_accounts_map.entry(tx_event.get_client_id());
        match tx_event.get_tx_type().as_str() {
            "deposit" => {
                if let Some(valid_tx_amount) = tx_event.get_tx_amount() {
                    match client_account_entry {
                        Entry::Occupied(mut occupied_entry) => {
                            if let Err(lock_error) = occupied_entry
                                .get_mut()
                                .deposit_to_account(&valid_tx_amount) {
                                         // Add context with client_id and tx_id
                                return Err(lock_error)
                                    .with_context(|| format!("for client {} and tx {}", tx_event.get_client_id(), tx_event.tx));
                            }
                        }
                        Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(UserAccountDetails::new(&valid_tx_amount));
                        }
                    }
                    let transaction_tracker_entry = TransactionTrackerEntry {
                        state: TransactionState::Deposit,
                        amount: valid_tx_amount,
                        client_id: tx_event.get_client_id(),
                    };
                    self.transaction_store
                        .record_transaction(tx_event.tx, transaction_tracker_entry);
                    Ok(())
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
                            Ok(_) => {
                                let transaction_tracker_entry = TransactionTrackerEntry {
                                    state: TransactionState::Withdraw,
                                    amount: valid_tx_amount,
                                    client_id: tx_event.get_client_id(),
                                };
                                self.transaction_store
                                    .record_transaction(tx_event.tx, transaction_tracker_entry);
                                Ok(())
                            }
                            Err(withdraw_error) => {
                                return Err(withdraw_error).with_context(|| format!("for client {} and tx {}", tx_event.get_client_id(), tx_event.tx));
                            }
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
            "dispute" => {
                // Fetch the transaction from the transaction store
                match self
                    .transaction_store
                    .get_valid_tx_and_create_dispute(&tx_event.tx)
                {
                    Ok((_client_id, amount_to_hold)) => {
                        if let Entry::Occupied(mut occupied_entry) = client_account_entry {
                            match occupied_entry
                                .get_mut()
                                .dispute_and_hold_funds(&amount_to_hold) {
                                    Ok(_) => Ok(()),
                                    Err(dispute_adjustment_error) => Err(dispute_adjustment_error)
                                }
                        } else {
                            Err(anyhow::format_err!(
                        "Account doesn't exist cannot dispute events that are not linked to a valid client id"
                    ))
                        }
                    }
                    Err(dispute_creation_error) => Err(dispute_creation_error),
                }
            },
            "resolve" => {
                match self
                    .transaction_store
                    .validate_dispute_state_of_tx_for_resolution_or_chargeback(&tx_event.tx)
                {
                    Ok((_client_id, amount_to_resolve)) => {
                        if let Entry::Occupied(mut occupied_entry) = client_account_entry {
                            match occupied_entry
                                .get_mut()
                                .resolve_dispute(&amount_to_resolve) {
                                    Ok(_) => Ok(()),
                                    Err(resolution_adjustment_error) => Err(resolution_adjustment_error)
                                }
                        } else {
                            Err(anyhow::format_err!(
                        "Account doesn't exist cannot dispute events that are not linked to a valid client id"
                    ))
                        }
                    }
                    Err(resolution_error) => Err(resolution_error),
                }
            },
            "chargeback" => {
                 match self
                    .transaction_store
                    .validate_dispute_state_of_tx_for_resolution_or_chargeback(&tx_event.tx)
                {
                    Ok((_client_id, amount_to_resolve)) => {
                        if let Entry::Occupied(mut occupied_entry) = client_account_entry {
                            match occupied_entry
                                .get_mut()
                                .perform_chargeback(&amount_to_resolve) {
                                    Ok(_) => Ok(()),
                                    Err(dispute_adjustment_error) => Err(dispute_adjustment_error)
                                }
                        } else {
                            Err(anyhow::format_err!(
                        "Account doesn't exist cannot dispute events that are not linked to a valid client id"
                    ))
                        }
                    }
                    Err(dispute_creation_error) => Err(dispute_creation_error),
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
                client_id, account.available, account.held, account.total, account.locked
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{transaction_handler::types::TxEvent, CacheHandler};

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
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
    }

    #[test]
    fn no_current_entry_exists_withdraw() {
        let mut cache_handler = CacheHandler::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "withdrawal".to_string(),
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
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);

        let test_deposit_tx_event_two = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 2,
            amount: Some(5.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_two);
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 6.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details.locked, false);
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
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);

        let test_withdraw_tx_event_two = TxEvent {
            tx_type: "withdrawal".to_string(),
            client: 1,
            tx: 2,
            amount: Some(0.5),
        };

        let result_under_test = cache_handler.handle_account_update(&test_withdraw_tx_event_two);
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 0.5);
        assert_eq!(account_details.total, 0.5);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
    }

    #[test]
    fn simple_withdrawal_insufficient_funds() {
        let mut cache_handler = CacheHandler::new();
        assert_eq!(cache_handler.transaction_store.transactions.len(), 0);
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
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);

        let test_withdraw_tx_event_two = TxEvent {
            tx_type: "withdrawal".to_string(),
            client: 1,
            tx: 2,
            amount: Some(2.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_withdraw_tx_event_two);
        let msg = format!("{:#}", result_under_test.unwrap_err()); 
        assert!(msg.contains("for client 1 and tx 2"));
        assert!(msg.contains("Insufficient funds to perform withdrawal"));

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
    }

    #[test]
    fn check_dispute_of_funds() {
        let mut cache_handler = CacheHandler::new();
        assert_eq!(cache_handler.transaction_store.transactions.len(), 0);
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
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 1);

        let test_deposit_tx_event_two = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 2,
            amount: Some(5.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_two);
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 6.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 2);

        let test_deposit_tx_event_three = TxEvent {
            tx_type: "dispute".to_string(),
            client: 1,
            tx: 2,
            amount: None
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_three);
        assert!(result_under_test.is_ok());
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 5.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 2);
    }


    #[test]
    fn check_deposit_dispute_resolution_flow() {
        let mut cache_handler = CacheHandler::new();
        assert_eq!(cache_handler.transaction_store.transactions.len(), 0);
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
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 1);

        let test_deposit_tx_event_two = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 2,
            amount: Some(5.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_two);
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 6.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 2);

        let test_deposit_tx_event_three = TxEvent {
            tx_type: "dispute".to_string(),
            client: 1,
            tx: 2,
            amount: None
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_three);
        assert!(result_under_test.is_ok());
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 5.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 2);

        // Check resolution here

        let test_deposit_tx_event_three = TxEvent {
            tx_type: "resolve".to_string(),
            client: 1,
            tx: 2,
            amount: None
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_three);
        assert!(result_under_test.is_ok());
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 6.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 2);

    }

    #[test]
    fn check_deposit_dispute_and_chargeback_flow() {
        let mut cache_handler = CacheHandler::new();
        assert_eq!(cache_handler.transaction_store.transactions.len(), 0);
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
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 1);

        let test_deposit_tx_event_two = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 2,
            amount: Some(5.0),
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_two);
        assert!(result_under_test.is_ok());

        assert_eq!(cache_handler.user_accounts_map.len(), 1);
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 6.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 2);

        let test_deposit_tx_event_three = TxEvent {
            tx_type: "dispute".to_string(),
            client: 1,
            tx: 2,
            amount: None
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_three);
        assert!(result_under_test.is_ok());
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 5.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 2);

        let test_deposit_tx_event_three = TxEvent {
            tx_type: "chargeback".to_string(),
            client: 1,
            tx: 2,
            amount: None
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_three);
        assert!(result_under_test.is_ok());
        let entry_under_test = cache_handler.user_accounts_map.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details.locked, true);
        assert_eq!(account_details.held, 0.0);
        assert_eq!(cache_handler.transaction_store.transactions.len(), 2);

         let test_deposit_tx_event_four = TxEvent {
            tx_type: "withdrawal".to_string(),
            client: 1,
            tx: 3,
            amount: Some(1.0)
        };

        let result_under_test = cache_handler.handle_account_update(&test_deposit_tx_event_four);
        assert!(result_under_test.is_err());
        let msg = format!("{:#}", result_under_test.unwrap_err()); 
        assert!(msg.contains("for client 1 and tx 3"));
        assert!(msg.contains("User account is locked"));

    }

    
}
