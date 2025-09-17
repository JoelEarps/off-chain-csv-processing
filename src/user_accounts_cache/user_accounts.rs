
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
}
