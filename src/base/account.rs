use serde::Deserialize;
//use super::paginated::Paginated;
use super::pagination::Pagination;
use super::money_hash::MoneyHash;

#[derive(Deserialize, Debug)]
pub struct AccountsResponse {
    pub pagination: Pagination,
    pub data: Vec<Account>,
}

#[derive(Deserialize, Debug)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub color: String,
    pub sort_index: i32,
    pub exponent: i32,
    #[serde(rename="type")]
    pub type_currency: String,
    pub address_regex: Option<String>,
    pub asset_id: Option<String>,
    pub slug: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub primary: bool,
    #[serde(rename="type")]
    pub type_account: String,
    pub currency: Currency,
    pub balance: MoneyHash,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub resource: String,
    pub resource_path: String,
}

// impl Paginated for AccountsResponse {
//     fn get_next_uri(&self) -> Option<String> {
//         return self.pagination.next_uri;
//     }

//     fn get_data<AccountsResponse, Account>(&self) -> Vec<Account> {
//         return self.data;
//     }
// }