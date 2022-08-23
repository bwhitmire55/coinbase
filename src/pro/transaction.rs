use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TransactionDetail {
    pub order_id: Option<String>,
    pub product_id: Option<String>,
    pub trade_id: Option<String>,
    pub to: Option<String>,
    pub from: Option<String>,
    pub profile_transfer_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub id: String,
    pub amount: String,
    pub created_at: String,
    pub balance: String,
    #[serde(rename="type")]
    pub type_transaction: String,
    pub details: Option<TransactionDetail>,
}