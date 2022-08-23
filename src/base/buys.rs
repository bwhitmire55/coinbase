use serde::Deserialize;
use super::money_hash::MoneyHash;
use super::pagination::Pagination;

#[derive(Deserialize, Debug)]
pub struct PaymentHash {
    pub id: String,
    pub resource: String,
    pub resource_path: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionHash {
    pub id: String,
    pub resource: String,
    pub resource_path: String,
}

#[derive(Deserialize, Debug)]
pub struct UnitPrice {
    pub amount: String,
    pub currency: String,
    pub scale: i32,
}

#[derive(Deserialize, Debug)]
pub struct Buy {
    pub id: String,
    pub status: String,
    pub payment_method: PaymentHash,
    pub transaction: Option<TransactionHash>,
    pub user_reference: Option<String>,
    pub amount: MoneyHash,
    pub total: MoneyHash,
    pub unit_price: Option<UnitPrice>,
    pub subtotal: MoneyHash,
    pub fee: MoneyHash,
    pub created_at: String,
    pub updated_at: String,
    pub resource: String,
    pub resource_path: String,
    pub committed: bool,
    pub payout_at: Option<String>,
    pub instant: bool,
    pub hold_until: Option<String>,
    pub hold_days: Option<i32>,
    pub idem: Option<String>,
    pub next_step: Option<String>,
    pub is_first_buy: Option<bool>,
    pub requires_completion_step: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct BuysResponse {
    pub pagination: Pagination,
    pub data: Vec<Buy>,
}