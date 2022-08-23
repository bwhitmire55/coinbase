use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MoneyHash {
    pub amount: String,
    pub currency: String,
}