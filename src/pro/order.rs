use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Order {
    pub trade_id: i32,
    pub product_id: String,
    pub order_id: String,
    pub user_id: String,
    pub profile_id: String,
    pub liquidity: String,
    pub price: String,
    pub size: String,
    pub fee: String,
    pub created_at: String,
    pub side: String,
    pub settled: bool,
    pub usd_volume: String,
}