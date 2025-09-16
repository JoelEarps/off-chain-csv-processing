pub(crate) struct UserAccountDetails {
    pub(crate) available: f64,
    pub(crate) held: f64,
    pub(crate) total: f64,
    pub(crate) _locked: bool,
}

impl UserAccountDetails {
    /// Only can be created with a deposit to an account that doesn't exist in the current account cache
    pub(crate) fn new(deposited_amount: &f64) -> Self {
        Self {
            available: *deposited_amount,
            held: 0.0,
            total: *deposited_amount,
            _locked: false,
        }
    }

    pub(crate) fn deposit_to_account(&mut self, deposited_amount: &f64) {
        self.available += deposited_amount;
        self.total = self.available + self.held;
    }

    /// Unlike deposit, this function returns a result, this is because it is not possible more from an account than possible
    /// If a client does not have sufficient available funds the withdrawal should fail and the total amount of funds should not change
    pub(crate) fn withdraw_from_account(&mut self, withdraw_amount: &f64) -> anyhow::Result<()> {
        if self.available < *withdraw_amount {
            Err(anyhow::anyhow!("Insufficient funds to perform withdrawal"))
        } else {
            self.available -= withdraw_amount;
            self.total = self.available + self.held;
            Ok(())
        }
    }

    /// This function manipulates the held values when being disputed
    /// There are two scenarios checked here for unknown failures:
    /// 1. The dispute amount is larger than available - this is an unknown scenario 
    /// 2. The total amount of funds has changed, which again should not be happening
    pub(crate) fn dispute_and_hold_funds(&mut self, hold_amount: &f64) -> anyhow::Result<()> {
        if self.available < *hold_amount {
            Err(anyhow::anyhow!("Unknown error - dispute amount is larger than available amount, this should not be happening?"))
        } else {
            self.available -= hold_amount;
            self.held += hold_amount;
            
            let old_total = self.total;
            let new_total = self.held + self.available;

            if old_total == new_total {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Unknown error - total funds in account has now changed, this should not be happening?"))
            }
        }
    }

    pub(crate) fn resolve_dispute(&mut self, amount_to_resolve: &f64) -> anyhow::Result<()> {
         if self.held < *amount_to_resolve {
            Err(anyhow::anyhow!("Unknown error - cannot resolve more funds than are being held"))
        } else {
            self.held -= amount_to_resolve;
            self.available += amount_to_resolve;
            
            let old_total = self.total;
            let new_total = self.held + self.available;

            if old_total == new_total {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Unknown error - total funds in account has now changed, this should not be happening?"))
            }
        }
    }
}

#[cfg(test)]
mod user_account_tests {
    use super::UserAccountDetails;

    #[rstest::rstest]
    #[case(5.0, vec![5.0], 10.0, 10.0)]
    #[case(5.0, vec![5.0, 10.0, 22.5], 42.5, 42.5)]
    fn user_account_details_create_and_deposit(
        #[case] initial_deposit: f64,
        #[case] deposit_sequence: Vec<f64>,
        #[case] expected_end_available: f64,
        #[case] expected_end_total: f64,
    ) {
        let mut account_under_test = UserAccountDetails::new(&initial_deposit);
        assert_eq!(account_under_test.available, initial_deposit);
        assert_eq!(account_under_test.total, initial_deposit);
        assert_eq!(account_under_test._locked, false);
        assert_eq!(account_under_test.held, 0.0);
        for deposit in deposit_sequence {
            account_under_test.deposit_to_account(&deposit);
        }
        assert_eq!(account_under_test.available, expected_end_available);
        assert_eq!(account_under_test.total, expected_end_total);
        assert_eq!(account_under_test._locked, false);
        assert_eq!(account_under_test.held, 0.0);
    }

    #[rstest::rstest]
    #[case(5.0, vec![5.0], vec![true], 0.0, 0.0)]
    #[case(60.0, vec![5.0, 10.0, 22.5], vec![true, true, true], 22.5, 22.5)]
    // In sufficent funds
    #[case(60.0, vec![20.0, 50.0], vec![true, false], 40.0, 40.0)]
    fn user_account_details_create_and_withdraw(
        #[case] initial_deposit: f64,
        #[case] withdrawal_sequence: Vec<f64>,
        #[case] success_sequence: Vec<bool>,
        #[case] expected_end_available: f64,
        #[case] expected_end_total: f64,
    ) {
        let mut account_under_test = UserAccountDetails::new(&initial_deposit);
        assert_eq!(account_under_test.available, initial_deposit);
        assert_eq!(account_under_test.total, initial_deposit);
        assert_eq!(account_under_test._locked, false);
        assert_eq!(account_under_test.held, 0.0);
        for i in 0..withdrawal_sequence.len() {
            assert_eq!(
                account_under_test
                    .withdraw_from_account(&withdrawal_sequence[i])
                    .is_ok(),
                success_sequence[i]
            );
        }
        assert_eq!(account_under_test.available, expected_end_available);
        assert_eq!(account_under_test.total, expected_end_total);
        assert_eq!(account_under_test._locked, false);
        assert_eq!(account_under_test.held, 0.0);
    } 


    #[rstest::rstest]
    #[case(5.0, 2.5)]
    #[case(100.0, 22.5)]
    // In sufficent funds
    #[case(60.0, 40.0)]
    fn deposit_to_resolution_to_dispute(
         #[case] initial_deposit: f64,
         #[case] dispute_amount: f64,
    ){
        let mut account_under_test = UserAccountDetails::new(&initial_deposit);
        assert_eq!(account_under_test.available, initial_deposit);
        assert_eq!(account_under_test.total, initial_deposit);
        assert_eq!(account_under_test._locked, false);
        assert_eq!(account_under_test.held, 0.0);

        let dispute_result = account_under_test.dispute_and_hold_funds(&dispute_amount);
        assert!(dispute_result.is_ok());
        assert_eq!(account_under_test.available, initial_deposit - dispute_amount);
        assert_eq!(account_under_test.total, initial_deposit);
        assert_eq!(account_under_test._locked, false);
        assert_eq!(account_under_test.held, 0.0 + dispute_amount);

        let resolve_result = account_under_test.resolve_dispute(&dispute_amount);
        assert!(resolve_result.is_ok());
        assert_eq!(account_under_test.available, initial_deposit);
        assert_eq!(account_under_test.total, initial_deposit);
        assert_eq!(account_under_test._locked, false);
        assert_eq!(account_under_test.held, 0.0);
    }
}
