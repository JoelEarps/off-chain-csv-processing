use std::collections::HashMap;

pub type TransactionTracker = HashMap<TxID, TransactionTrackerEntry>;

use serde::Deserialize;

pub type TransactionStateValueReturn = (ClientID, f64);

pub struct TransactionTrackerEntry {
    pub(crate) state: TransactionState,
    pub(crate) amount: f64,
    pub(crate) client_id: ClientID,
}

pub(crate) enum TransactionState {
    Deposit,
    Withdraw,
    Dispute,
    Resolved,
    _ChargeBack,
}

pub(crate) type ClientID = u16;
pub(crate) type TxID = u32;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct TxEvent {
    #[serde(rename = "type")]
    pub(crate) tx_type: TxType,
    pub(crate) client: ClientID,
    pub(crate) tx: TxID,
    pub(crate) amount: Option<f64>,
}

impl TxEvent {
    pub(crate) fn get_tx_type(&self) -> TxType {
        self.tx_type.clone()
    }

    pub(crate) fn get_tx_amount(&self) -> Option<f64> {
        self.amount
    }

    pub(crate) fn get_client_id(&self) -> ClientID {
        self.client
    }
}
