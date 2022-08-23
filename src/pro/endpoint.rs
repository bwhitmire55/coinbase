pub enum Endpoint {
    Accounts,
    Account,
    AccountLedger,
    Ticker,
    FilledOrders,
    Transfers,
    Order,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::Accounts => "/accounts".to_string(),
            Self::Account => "/accounts/{}".to_string(),
            Self::AccountLedger => "/accounts/{}/ledger".to_string(),
            Self::Ticker => "/products/{}/ticker".to_string(),
            Self::FilledOrders => "/fills?product_id={}".to_string(),
            Self::Transfers => "/accounts/{}/transfers".to_string(),
            Self::Order => "/fills?order_id={}".to_string(),
        }
    }
}