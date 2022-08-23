pub enum Endpoint {
    Accounts,
    Buys,
    Transactions,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::Accounts => "/accounts".to_string(),
            Self::Buys => "/accounts/{}/buys".to_string(),
            Self::Transactions => "/accounts/{}/transactions".to_string(),
        }
    }
}