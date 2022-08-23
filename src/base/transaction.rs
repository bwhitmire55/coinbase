use serde::Deserialize;
use super::pagination::Pagination;
use super::money_hash::MoneyHash;
use super::buys::Buy;

#[derive(Deserialize, Debug)]
pub struct TransactionDetails {
    pub title: String,
    pub subtitle: Option<String>,
    pub header: Option<String>,
    pub health: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionNetwork {
    pub status: Option<String>,
    pub status_description: Option<String>,
    pub hash: Option<String>,
    pub transaction_url: Option<String>,
    pub transaction_fee: Option<MoneyHash>,
    pub transaction_amount: Option<MoneyHash>,
    pub confirmations: Option<i32>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AddressInfo {
    pub address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionOrder {
    pub id: Option<String>,
    pub resource: Option<String>,
    pub resource_path: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub currency: Option<String>,
    pub address_info: Option<AddressInfo>,
    pub address_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AdvancedFill {
    pub fill_price: String,
    pub product_id: String,
    pub order_id: String,
    pub commission: String,
}

#[derive(Deserialize, Debug)]
pub struct Trade {
    pub id: String,
    pub status: String,
    pub transaction: TradeTransaction,
    pub user_reference: String,
    pub created_at: String,
    pub updated_at: String,
    pub resource: String,
    pub resource_path: Option<String>,
    pub payment_method: TradePaymentMethod,
    pub committed: bool,
    pub payout_at: String,
    pub fee: MoneyHash,
    pub idem: String,
    pub next_step: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TradeTransaction {
    pub id: String,
    pub resource: String,
    pub resource_path: String,
}

#[derive(Deserialize, Debug)]
pub struct TradePaymentMethod {
    pub id: String,
    pub resource: String,
    pub resource_path: String,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub id: String,
    #[serde(rename="type")]
    pub type_transaction: String,
    pub status: String,
    pub amount: MoneyHash,
    pub native_amount: MoneyHash,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub resource: String,
    pub resource_path: String,
    pub advanced_trade_fill: Option<AdvancedFill>,
    pub details: TransactionDetails,
    pub network: Option<TransactionNetwork>,
    pub instant_exchange: Option<bool>,
    pub hide_native_amount: Option<bool>,
    pub to: Option<TransactionOrder>,
    pub from: Option<TransactionOrder>,
    pub buy: Option<Buy>,
//    pub sell: Option<Sell>,
    pub trade: Option<Trade>,
    pub address: Option<TransactionOrder>,
    pub application: Option<TransactionOrder>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionsResponse {
    pub pagination: Pagination,
    pub data: Vec<Transaction>,
}