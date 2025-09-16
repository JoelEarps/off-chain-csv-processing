use crate::transaction_handler::types::{
    TransactionState, TransactionStateValueReturn, TransactionTracker, TransactionTrackerEntry,
};

pub(crate) struct TransactionCache {
    transactions: TransactionTracker,
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
            Err(anyhow::anyhow!("Transaction doesn't exist and there"))
        }

        // Change the entry to a dispute

        // Return amount to be removed/ added from user total
    }
}

#[cfg(test)]
mod tx_cache_tests {
    use crate::transaction_handler::types::{TransactionTracker, TransactionTrackerEntry};

    #[rstest::rstest]
    #[case(1, TransactionTrackerEntry {
            state: TransactionState::Withdraw,
                                amount: 1.0,
                                client_id: 2,
                            }, 1, false, None, Some("Cannot create a disupte in any state other than Deposit") )]
    fn check_dispute_transition(
        #[case] tx_id: u32,
        #[case] entry_to_insert: TransactionTrackerEntry,
        #[case] tx_to_fetch: u32,
        #[case] is_ok: bool,
        expected_success: Option<TransactionStateValueReturn>,
        expected_failure_message: Option<&str>,
    ) {
        let tx_tracker_under_test = TransactionTracker::new();
    }
}
