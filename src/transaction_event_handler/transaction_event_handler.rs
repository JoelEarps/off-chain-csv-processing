use serde::Deserialize;

pub(crate) enum _ValidTxTypes {
    Deposit,
    Withdraw,
}

pub(crate) type ClientID = u16;
pub(crate) type TxID = u32;

#[derive(Debug, Deserialize)]
pub struct TxEvent {
    #[serde(rename = "type")]
    pub(crate) tx_type: String,
    pub(crate) client: ClientID,
    pub(crate) tx: TxID,
    pub(crate) amount: Option<f64>,
}

impl TxEvent {
    pub(crate) fn get_tx_type(&self) -> String {
        self.tx_type.clone()
    }

    pub(crate) fn get_tx_amount(&self) -> Option<f64> {
        self.amount
    }

    pub(crate) fn get_client_id(&self) -> ClientID {
        self.client
    }
}
