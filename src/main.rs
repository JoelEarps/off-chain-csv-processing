use std::{collections::{hash_map::Entry, HashMap}, env, error::Error};

use csv_async::{AsyncReaderBuilder};
use serde::Deserialize;
use tokio::fs::File;
use tokio_stream::StreamExt;

enum ValidTxTypes {
    Deposit,
    Withdraw,
}

#[derive(Debug, Deserialize)]
struct TxEvent {
    #[serde(rename = "type")]
    // TODO: Turn into Enum for better matching and general maintainability
    tx_type: String,
    client: ClientID,
    tx: TxID,
    amount: Option<f64>,
}

struct UserAccountDetails {
    available: f64,
    held: f64,
    total: f64,
    locked: bool
}

impl UserAccountDetails {
    /// Only can be created with a deposit to an account that doesn't exist in the current account cache
    pub fn new(deposited_amount: &f64) -> Self {
        Self {
            available: *deposited_amount,
            held: 0.0,
            total: *deposited_amount,
            locked: false
        }
    }

    pub fn deposit_to_account(&mut self, deposited_amount: &f64) {
        self.available += deposited_amount;  
        self.total = self.available + self.held;
    }

    /// Unlike deposit, this function returns a result, this is because it is not possible more from an account than possible
    /// If a client does not have sufficient available funds the withdrawal should fail and the total amount of funds should not change
    pub fn withdraw_from_account(&mut self, withdraw_amount: &f64) -> anyhow::Result<()> {
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
    use crate::UserAccountDetails;


    #[rstest::rstest]
    #[case(5.0, vec![5.0], 10.0, 10.0)]
    #[case(5.0, vec![5.0, 10.0, 22.5], 42.5, 42.5)]
    fn user_account_details_create_and_deposit(#[case] initial_deposit: f64, #[case] deposit_sequence: Vec<f64>, #[case] expected_end_available: f64, #[case] expected_end_total: f64) {
        let mut account_under_test = UserAccountDetails::new(&initial_deposit);
        assert_eq!(account_under_test.available, initial_deposit);
        assert_eq!(account_under_test.total, initial_deposit);
        assert_eq!(account_under_test.locked, false);
        assert_eq!(account_under_test.held, 0.0);
        for deposit in deposit_sequence {
            account_under_test.deposit_to_account(&deposit);
        }
        assert_eq!(account_under_test.available, expected_end_available);
        assert_eq!(account_under_test.total, expected_end_total);
        assert_eq!(account_under_test.locked, false);
        assert_eq!(account_under_test.held, 0.0);
    }


    #[rstest::rstest]
    #[case(5.0, vec![5.0], vec![true], 0.0, 0.0)]
    #[case(60.0, vec![5.0, 10.0, 22.5], vec![true, true, true], 22.5, 22.5)]
    // In sufficent funds
    #[case(60.0, vec![20.0, 50.0], vec![true, false], 40.0, 40.0)]
    fn user_account_details_create_and_withdraw(#[case] initial_deposit: f64, #[case] withdrawal_sequence: Vec<f64>, #[case] success_sequence: Vec<bool>, #[case] expected_end_available: f64, #[case] expected_end_total: f64) {
        let mut account_under_test = UserAccountDetails::new(&initial_deposit);
        assert_eq!(account_under_test.available, initial_deposit);
        assert_eq!(account_under_test.total, initial_deposit);
        assert_eq!(account_under_test.locked, false);
        assert_eq!(account_under_test.held, 0.0);
        for i in 0..withdrawal_sequence.len() {
            assert_eq!(account_under_test.withdraw_from_account(&withdrawal_sequence[i]).is_ok(), success_sequence[i]);
        }
        assert_eq!(account_under_test.available, expected_end_available);
        assert_eq!(account_under_test.total, expected_end_total);
        assert_eq!(account_under_test.locked, false);
        assert_eq!(account_under_test.held, 0.0);
    }
}


// turn into associated types
// Same with errors
type ClientID = u16;
type TxID = u32;

type AccountStore = HashMap<ClientID, UserAccountDetails>;

/// TODO - create custom struct 
// impl std::fmt::Display for AccountStore {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

type ErrorReport = Vec<Box<dyn Error + 'static>>;
type ErrorReportAnyhow = Vec<anyhow::Error>;


/// Why move handling inside here
/// Reduction of duplicate logic
/// TODO: turn this into struct that holds this info, rather than passing round refs you can use references to self
fn handle_account_update(tx_event: &TxEvent, account_store: &mut AccountStore) -> anyhow::Result<()> {
    // Cannot do anything if account is null and the tx type is not a deposit, so handle
    let client_account_entry = account_store.entry(tx_event.client);
    match tx_event.tx_type.as_str() {
        "deposit" => {
             if let Some(valid_tx_amount) = tx_event.amount {
                        match client_account_entry {
                            Entry::Occupied(mut occupied_entry) => {
                                occupied_entry.get_mut().deposit_to_account(&valid_tx_amount);
                                Ok(())
                            },
                            Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert( UserAccountDetails::new(&valid_tx_amount));
                                Ok(())
                            },
                        } 
                    } else {
                        return Err(anyhow::format_err!("Cannot perform operation due to empty value, this should not be happening"))
            }
        },
        "withdraw" => {
            if let Entry::Occupied(mut occupied_entry) = client_account_entry {
                if let Some(valid_tx_amount) = tx_event.amount {
                    match occupied_entry.get_mut().withdraw_from_account(&valid_tx_amount) {
                        Ok(_) => Ok(()),
                        Err(withdraw_error) => Err(withdraw_error),
                    }
                } else {
                        return Err(anyhow::format_err!("Cannot perform operation due to empty value, this should not be happening"))
                }
            } else {
                Err(anyhow::format_err!("Account doesn't exist cannot withdraw from an un open account"))
            }
        }
        _ => Err(anyhow::format_err!("Unhandled event"))
    }
}

#[tokio::main]
async fn main()  -> anyhow::Result<()> {
    // Expect: cargo run -- transactions.csv > accounts.csv
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).expect("please provide CSV file path");
    let mut read_errors = ErrorReport::new();
    let mut client_account_store = AccountStore::new();
    let mut client_tx_errors = ErrorReportAnyhow::new();

    let file = File::open(input_path).await?;
    let rdr = AsyncReaderBuilder::new().trim(csv_async::Trim::All).create_deserializer(file);
    let mut records = rdr.into_deserialize::<TxEvent>();

    while let Some(record) = records.next().await {
        match record {
            Ok(transaction_event) => {

            match handle_account_update(&transaction_event,  &mut client_account_store) {
                Ok(_) => {
                    println!("Transaction handled successfully");
                },
                Err(error) => {
                    client_tx_errors.push(error);
                },
            } 
            },
            Err(error) => {
                read_errors.push(Box::new(error));
            } 
        }
    }
        
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{handle_account_update, AccountStore, TxEvent};


    // All test Scenarios for these unit tests can be found here:
    // docs/bdd-scenarios/deposits-and-withdrawals.feature
    #[test]
    fn no_current_account_entry_deposit() {
        let mut client_account_store_under_test = AccountStore::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = handle_account_update(&test_deposit_tx_event,  &mut client_account_store_under_test);
        assert!(result_under_test.is_ok());

        assert_eq!(client_account_store_under_test.len(), 1);
        let entry_under_test = client_account_store_under_test.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
    }

    #[test]
    fn no_current_entry_exists_withdraw() {
        let mut client_account_store_under_test = AccountStore::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "withdraw".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = handle_account_update(&test_deposit_tx_event,  &mut client_account_store_under_test);
        assert!(result_under_test.is_err());
        assert_eq!(result_under_test.unwrap_err().to_string(), "Account doesn't exist cannot withdraw from an un open account");

        assert_eq!(client_account_store_under_test.len(), 0);
        let entry_under_test = client_account_store_under_test.get(&1);
        assert!(entry_under_test.is_none());
    }

    #[test]
    fn simple_deposit_success() {
        let mut client_account_store_under_test = AccountStore::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = handle_account_update(&test_deposit_tx_event,  &mut client_account_store_under_test);
        assert!(result_under_test.is_ok());

        assert_eq!(client_account_store_under_test.len(), 1);
        let entry_under_test = client_account_store_under_test.get(&1);
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

        let result_under_test = handle_account_update(&test_deposit_tx_event_two,  &mut client_account_store_under_test);
        assert!(result_under_test.is_ok());

        assert_eq!(client_account_store_under_test.len(), 1);
        let entry_under_test = client_account_store_under_test.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 6.0);
        assert_eq!(account_details.total, 6.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);

    }

    #[test]
    fn simple_withdrawal_success() {

        let mut client_account_store_under_test = AccountStore::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = handle_account_update(&test_deposit_tx_event,  &mut client_account_store_under_test);
        assert!(result_under_test.is_ok());

        assert_eq!(client_account_store_under_test.len(), 1);
        let entry_under_test = client_account_store_under_test.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);

        let test_withdraw_tx_event_two = TxEvent {
            tx_type: "withdraw".to_string(),
            client: 1,
            tx: 2,
            amount: Some(0.5),
        };

        let result_under_test = handle_account_update(&test_withdraw_tx_event_two,  &mut client_account_store_under_test);
        assert!(result_under_test.is_ok());

        assert_eq!(client_account_store_under_test.len(), 1);
        let entry_under_test = client_account_store_under_test.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 0.5);
        assert_eq!(account_details.total, 0.5);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);

    }

    #[test]
    fn simple_withdrawal_insufficient_funds() {
        let mut client_account_store_under_test = AccountStore::new();
        let test_deposit_tx_event = TxEvent {
            tx_type: "deposit".to_string(),
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };

        let result_under_test = handle_account_update(&test_deposit_tx_event,  &mut client_account_store_under_test);
        assert!(result_under_test.is_ok());

        assert_eq!(client_account_store_under_test.len(), 1);
        let entry_under_test = client_account_store_under_test.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);

        let test_withdraw_tx_event_two = TxEvent {
            tx_type: "withdraw".to_string(),
            client: 1,
            tx: 2,
            amount: Some(2.0),
        };

        let result_under_test = handle_account_update(&test_withdraw_tx_event_two,  &mut client_account_store_under_test);
        assert_eq!(result_under_test.unwrap_err().to_string(), "Insufficient funds to perform withdrawal");


        assert_eq!(client_account_store_under_test.len(), 1);
        let entry_under_test = client_account_store_under_test.get(&1);
        assert!(entry_under_test.is_some());
        let account_details = entry_under_test.unwrap();
        assert_eq!(account_details.available, 1.0);
        assert_eq!(account_details.total, 1.0);
        assert_eq!(account_details.locked, false);
        assert_eq!(account_details.held, 0.0);
    }


}
