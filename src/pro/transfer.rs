use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TransferDetails {
    pub coinbase_account_id: Option<String>,
    pub coinbase_transaction_id: Option<String>,
    pub coinbase_payment_method_id: Option<String>,
    pub fee: Option<String>,
    pub subtotal: Option<String>,
    pub sent_to_address: Option<String>,
    pub crypto_address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Transfer {
    pub id: String,
    #[serde(rename="type")]
    pub type_transfer: String,
    pub created_at: String,
    pub completed_at: String,
    pub canceled_at: Option<String>,
    pub processed_at: Option<String>,
    pub user_nonce: Option<String>,
    pub amount: String,
    pub details: Option<TransferDetails>,
}