use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Ticker {
    pub ask: String,
    pub bid: String,
    pub volume: String,
    pub trade_id: i32,
    pub price: String,
    pub size: String,
    pub time: String,
}