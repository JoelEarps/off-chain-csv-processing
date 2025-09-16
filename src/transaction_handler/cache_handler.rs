use crate::transaction_handler::types::{
    TransactionState, TransactionStateValueReturn, TransactionTracker, TransactionTrackerEntry,
};

pub(crate) struct TransactionCache {
    pub(crate) transactions: TransactionTracker,
}

impl TransactionCache {
    pub fn new() -> Self {
        Self {
            transactions: TransactionTracker::new(),
        }
    }

    pub fn record_transaction(&mut self, tx_id: u32, entry: TransactionTrackerEntry) {
        self.transactions.insert(tx_id, entry);
    }

    pub fn _get_status(&self, tx_id: &u32) -> Option<&TransactionTrackerEntry> {
        self.transactions.get(tx_id)
    }

    pub fn get_status_mut(&mut self, tx_id: &u32) -> Option<&mut TransactionTrackerEntry> {
        self.transactions.get_mut(tx_id)
    }

    pub fn get_valid_tx_and_create_dispute(
        &mut self,
        tx_id: &u32,
    ) -> anyhow::Result<TransactionStateValueReturn> {
        if let Some(valid_entry) = self.get_status_mut(tx_id) {
            match valid_entry.state {
                TransactionState::Deposit => {
                    valid_entry.state = TransactionState::Dispute;
                    Ok((valid_entry.client_id, valid_entry.amount))
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Cannot create a disupte in any state other than Deposit"
                    ))
                }
            }
        } else {
            Err(anyhow::anyhow!(
                "Transaction doesn't exist and therefore can be ignored"
            ))
        }
    }

    pub fn resolution_of_dispute(
        &mut self,
        tx_id: &u32
    ) -> anyhow::Result<TransactionStateValueReturn>{
        if let Some(valid_entry) = self.get_status_mut(tx_id) {
            match valid_entry.state {
                TransactionState::Dispute => {
                    valid_entry.state = TransactionState::Resolved;
                    Ok((valid_entry.client_id, valid_entry.amount))
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Cannot resolve a transaction that is not in the dispute state"
                    ))
                }
            }
        } else {
            Err(anyhow::anyhow!(
                "Transaction doesn't exist and therefore can be ignored"
            ))
        }
    }
}

#[cfg(test)]
mod tx_cache_tests {
    use crate::transaction_handler::{
        cache_handler::TransactionCache,
        types::{TransactionState, TransactionStateValueReturn, TransactionTrackerEntry},
    };

    #[rstest::rstest]
    #[case(1,
        TransactionTrackerEntry {
            state: TransactionState::Withdraw,
            amount: 1.0,
            client_id: 2,
            }, 1, false, None, Some("Cannot create a disupte in any state other than Deposit") )]
    #[case(1,
        TransactionTrackerEntry {
            state: TransactionState::Deposit,
            amount: 1.0,
            client_id: 2,
            }, 1, true, Some((2, 1.0)), None )]
    #[case(1,
        TransactionTrackerEntry {
            state: TransactionState::Deposit,
            amount: 1.0,
            client_id: 2,
            }, 5, false, None, Some("Transaction doesn't exist and therefore can be ignored") )]
    fn check_dispute_transition(
        #[case] tx_id: u32,
        #[case] entry_to_insert: TransactionTrackerEntry,
        #[case] tx_to_fetch: u32,
        #[case] is_ok: bool,
        #[case] expected_success: Option<TransactionStateValueReturn>,
        #[case] expected_failure_message: Option<&str>,
    ) {
        let mut tx_tracker_under_test = TransactionCache::new();
        tx_tracker_under_test.record_transaction(tx_id, entry_to_insert);
        let transition_to_dispute_under_test =
            tx_tracker_under_test.get_valid_tx_and_create_dispute(&tx_to_fetch);
        if is_ok {
            assert!(transition_to_dispute_under_test.is_ok());
            assert_eq!(
                transition_to_dispute_under_test.unwrap(),
                expected_success.unwrap()
            );
        } else {
            assert!(transition_to_dispute_under_test.is_err());
            assert_eq!(
                transition_to_dispute_under_test.unwrap_err().to_string(),
                expected_failure_message.unwrap().to_string()
            )
        }
    }

    #[rstest::rstest]
    #[case(1,
        TransactionTrackerEntry {
            state: TransactionState::Withdraw,
            amount: 1.0,
            client_id: 2,
            }, 1, false, None, Some("Cannot resolve a transaction that is not in the dispute state") )]
    #[case(1,
         TransactionTrackerEntry {
            state: TransactionState::Deposit,
            amount: 1.0,
            client_id: 2,
            }, 1, false, None, Some("Cannot resolve a transaction that is not in the dispute state") )]
    #[case(1,
        TransactionTrackerEntry {
            state: TransactionState::Dispute,
            amount: 1.0,
            client_id: 2,
            }, 5, false, None, Some("Transaction doesn't exist and therefore can be ignored") )]
    #[case(1,
        TransactionTrackerEntry {
            state: TransactionState::Dispute,
            amount: 1.0,
            client_id: 2,
            }, 1, true, Some((2, 1.0)), None )]
    fn check_resolution_of_dispute  (
        #[case] tx_id: u32,
        #[case] entry_to_insert: TransactionTrackerEntry,
        #[case] tx_to_fetch: u32,
        #[case] is_ok: bool,
        #[case] expected_success: Option<TransactionStateValueReturn>,
        #[case] expected_failure_message: Option<&str>) {
        
        let mut tx_tracker_under_test = TransactionCache::new();
        tx_tracker_under_test.record_transaction(tx_id, entry_to_insert);

        let transition_to_dispute_under_test =
            tx_tracker_under_test.resolution_of_dispute(&tx_to_fetch);

        if is_ok {
            assert!(transition_to_dispute_under_test.is_ok());
            assert_eq!(
                transition_to_dispute_under_test.unwrap(),
                expected_success.unwrap()
            );
        } else {
            assert!(transition_to_dispute_under_test.is_err());
            assert_eq!(
                transition_to_dispute_under_test.unwrap_err().to_string(),
                expected_failure_message.unwrap().to_string()
            )
        }
    }
}
