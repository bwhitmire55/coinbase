use super::account::*;
use super::buys::*;
use super::endpoint::*;
use super::paginated::*;
use super::transaction::*;
use super::APIError;
use super::CBErrorResponse;
use super::Url;

use std::{time::{SystemTime, UNIX_EPOCH}};
use sha2::Sha256;
use hmac::{Hmac, Mac};

pub struct Client {
    cb_access_key: String,
    cb_access_secret: String,
    cb_version: String,
}

impl Client {
    pub fn new(api_key: &str, api_secret: &str, api_version: &str) -> Client {
        Client {
            cb_access_key: api_key.to_string(),
            cb_access_secret: api_secret.to_string(),
            cb_version: api_version.to_string(),
        }
    }

    pub fn get_api_key(&self) -> &str {
        &self.cb_access_key
    }

    pub fn get_api_secret(&self) -> &str {
        &self.cb_access_secret
    }

    pub fn get_api_version(&self) -> &str {
        &self.cb_version
    }
    
    fn prepare_url(&self, endpoint: Endpoint, account_id: &str) -> Url {
        const BASE_URL: &str = "https://api.coinbase.com/v2";
        let mut url = Url::new((BASE_URL.to_owned() + &endpoint.to_string()).as_str());
        url.fill_data(account_id);
        url
    }
    
    fn coinbase_authentication(&self, timestamp: u64, path: &str) -> String {
        let message = format!("{}{}{}{}", timestamp, "GET", path, "");
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.cb_access_secret.as_bytes()).expect("");
    
        mac.update(message.as_bytes());
        mac.finalize().into_bytes().iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<_>>().join("")
    }
    
    fn build_coinbase_request(&self, url: &Url, client: &reqwest::Client) -> Result<reqwest::Request, APIError> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let signature = self.coinbase_authentication(timestamp, url.path().as_str());
    
        let request = client
            .request(reqwest::Method::GET, url.get())
            .header("CB-ACCESS-KEY", &self.cb_access_key)
            .header("CB-ACCESS-SIGN", signature)
            .header("CB-ACCESS-TIMESTAMP", timestamp)
            .header("CB-VERSION", &self.cb_version)
            .header("User-Agent", "Yee/1.0")
            .build()
            .map_err(|e| APIError::RequestFailed(e))?;
    
        Ok(request)
    }

    pub async fn fetch_resource<T, P>(&self, path: &str) -> Result<Vec<P>, APIError> 
        where T: serde::de::DeserializeOwned + Paginated
    {
        const BASE_URL: &str = "https://api.coinbase.com";
        let url = Url::new(format!("{}{}", BASE_URL, path).as_str());
        let client = reqwest::Client::new();
        let request = self.build_coinbase_request(&url, &client)?;

        let response = client.execute(request).await?;
        let mut result: Vec<P> = Vec::<P>::new();

        match response.status().as_str() {
            "200" => {
                let parsed: T = response.json().await.map_err(|e| APIError::RequestFailed(e))?;
                for obj in parsed.get_data::<T, P>() {
                    result.push(obj);
                }

                match parsed.get_next_uri() {
                    Some(uri) => {
                        // ...
                    },
                    None => {

                    },
                }
            },
            code => {
                let result: CBErrorResponse = response.json().await?;
                return Err(APIError::CoinbaseError(format!("{} - {}", code, result.errors[0].message)));
            },
        }

        Ok(result)
    }

    pub async fn print_response(&self, path: &String) -> Result<String, APIError> {
        const BASE_URL: &str = "https://api.coinbase.com";
        let url = Url::new(format!("{}{}", BASE_URL, path).as_str());
        let client = reqwest::Client::new();
        let request = self.build_coinbase_request(&url, &client)?;
        let response = client.execute(request).await.map_err(|e| APIError::RequestFailed(e))?;
        Ok(response.text().await?)
    }

    // use streams...
    pub async fn fetch_accounts(&self) -> Result<Vec<Account>, APIError> {
        // return self.fetch_resource::<AccountsResponse>("/v2/accounts").await?;

        const BASE_URL: &str = "https://api.coinbase.com";
        let mut accounts = Vec::<Account>::new();
        let mut url: Url = Url::new("");
    
        loop {
            if url.get().is_empty() {
                url = self.prepare_url(Endpoint::Accounts, "");
            }
    
            let client = reqwest::Client::new();
            let request = self.build_coinbase_request(&url, &client)?;
            
            let response: AccountsResponse = client 
                .execute(request)
                .await?
                .json()
                .await
                .map_err(|e| APIError::RequestFailed(e))?;
            
    
            for account in response.data {
                accounts.push(account);
            }
        
            match response.pagination.next_uri {
                Some(uri) => {
                    url = Url::new(format!("{}{}", BASE_URL, uri).as_str());
                },
                None => { break; },
            }
        }
    
        Ok(accounts)
    }

    pub async fn fetch_account_transactions(&self, account_id: &String) -> Result<Vec<Transaction>, APIError> {
        const BASE_URL: &str = "https://api.coinbase.com";
        let mut transactions = Vec::<Transaction>::new();
        let mut url: Url = Url::new("");
    
        loop {
            if url.get().is_empty() {
                url = self.prepare_url(Endpoint::Transactions, account_id)
                    .add_param("expand[]", "buy")
                    .add_param("expand[]", "sell") 
                    .add_param("expand[]", "trade");
            }
    
            let client = reqwest::Client::new();
            let request = self.build_coinbase_request(&url, &client)?;
            
            let response: TransactionsResponse = client 
                .execute(request)
                .await?
                .json()
                .await
                .map_err(|e| APIError::RequestFailed(e))?;            
    
            for transaction in response.data {
                transactions.push(transaction);
            }
        
            match response.pagination.next_uri {
                Some(uri) => {
                    url = Url::new(format!("{}{}", BASE_URL, uri).as_str());
                },
                None => { break; },
            }
        }
    
        Ok(transactions)
    }

    pub async fn fetch_account_buys(&self, account_id: &String) -> Result<Vec<Buy>, APIError> {
        const BASE_URL: &str = "https://api.coinbase.com";
        let mut buys = Vec::<Buy>::new();
        let mut url: Url = Url::new("");

        loop {
            if url.get().is_empty() {
                url = self.prepare_url(Endpoint::Buys, account_id);
            }

            let client = reqwest::Client::new();
            let request = self.build_coinbase_request(&url, &client)?;

            let response: BuysResponse = client 
                .execute(request) 
                .await? 
                .json() 
                .await 
                .map_err(|e| APIError::RequestFailed(e))?;

            for buy in response.data {
                buys.push(buy);
            }

            match response.pagination.next_uri {
                Some(uri) => {
                    url = Url::new(format!("{}{}", BASE_URL, uri).as_str());
                },
                None => { break; },
            }
        }

        Ok(buys)
    }
}