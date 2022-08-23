use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub currency: String,
    pub balance: String,
    pub available: String,
    pub hold: String,
    pub profile_id: String,
    pub trading_enabled: bool,
}