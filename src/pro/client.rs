use super::account::*;
use super::APIError;
use super::endpoint::*;
use super::order::*;
use super::ticker::*;
use super::transaction::*;
use super::transfer::*;
use super::Url;

use std::time::{SystemTime, UNIX_EPOCH};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use base64::{encode, decode};

pub struct Client {
    cb_access_key: String,
    cb_access_secret: String,
    cb_access_passphrase: String,
}

impl Client {
    pub fn new(api_key: &str, api_secret: &str, api_passphrase: &str) -> Client {
        Client {
            cb_access_key: api_key.to_string(),
            cb_access_secret: api_secret.to_string(),
            cb_access_passphrase: api_passphrase.to_string(),
        }
    }

    pub fn get_api_key(&self) -> &str {
        &self.cb_access_key
    }

    pub fn get_api_secret(&self) -> &str {
        &self.cb_access_secret
    }

    pub fn get_api_passphrase(&self) -> &str {
        &self.cb_access_passphrase
    }

    fn prepare_url(&self, endpoint: Endpoint, account_id: &str) -> Url {
        const BASE_URL: &str = "https://api.exchange.coinbase.com";
        let mut url = Url::new((BASE_URL.to_owned() + &endpoint.to_string()).as_str());
        url.fill_data(account_id);
        url
    }

    fn coinbase_authentication(&self, timestamp: u64, path: &str) -> String {
        let message = format!("{}{}{}{}", timestamp, "GET", path, "");
        let mut mac = Hmac::<Sha256>::new_from_slice(&decode(&self.cb_access_secret).unwrap()[..]).expect("");

        mac.update(message.as_bytes());
        encode(mac.finalize().into_bytes())
    }

    fn build_coinbase_request(&self, url: &Url, client: &reqwest::Client) -> Result<reqwest::Request, APIError> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let signature = self.coinbase_authentication(timestamp, url.path().as_str());

        let request = client
            .request(reqwest::Method::GET, url.get())
            .header("CB-ACCESS-KEY", &self.cb_access_key)
            .header("CB-ACCESS-SIGN", signature)
            .header("CB-ACCESS-TIMESTAMP", timestamp)
            .header("CB-ACCESS-PASSPHRASE", &self.cb_access_passphrase)
            .header("CB-VERSION", "2022-07-14")
            .header("User-Agent", "Yee/1.0")
            .build()
            .map_err(|e| APIError::RequestFailed(e))?;

        Ok(request)
    }

    async fn process_request(&self, client: &reqwest::Client, request: reqwest::Request) -> Result<reqwest::Response, APIError> {
        let response = client.execute(request).await?;

        if response.status() != reqwest::StatusCode::OK {
            println!("CBError [{}]: {:#?}", response.status().as_str(), response.text().await);
            return Err(APIError::BadRequest("CBError"))
        }

        Ok(response)
    }

    pub async fn fetch_accounts(&self) -> Result<Vec<Account>, APIError> {
        let url = self.prepare_url(Endpoint::Accounts, "");
        let client = reqwest::Client::new();
        let request = self.build_coinbase_request(&url, &client)?;

        let response: Vec<Account> = client 
            .execute(request)
            .await?
            .json()
            .await
            .map_err(|e| APIError::RequestFailed(e))?;

        if response.is_empty() {
            return Err(APIError::BadRequest("No data received from server"));
        }

        Ok(response)
    }

    pub async fn fetch_account(&self, account_id: &String) -> Result<Account, APIError> {
        let url = self.prepare_url(Endpoint::Account, account_id.as_str());
        let client = reqwest::Client::new();
        let request = self.build_coinbase_request(&url, &client)?;

        let response: Account = client 
            .execute(request) 
            .await? 
            .json() 
            .await 
            .map_err(|e| APIError::RequestFailed(e))?;

        Ok(response)
    }

    pub async fn fetch_account_ledger(&self, account_id: &String) -> Result<Vec<Transaction>, APIError> {
        let url = self.prepare_url(Endpoint::AccountLedger, account_id.as_str());
        let client = reqwest::Client::new();
        let request = self.build_coinbase_request(&url, &client)?;
        
        let response: Vec<Transaction> = client 
            .execute(request)
            .await?
            .json()
            .await
            .map_err(|e| APIError::RequestFailed(e))?;

        Ok(response)
    }

    pub async fn fetch_product_ticker(&self, product_id: &String) -> Result<Ticker, APIError> {
        let url = self.prepare_url(Endpoint::Ticker, product_id.as_str());
        let client = reqwest::Client::new();
        let request = self.build_coinbase_request(&url, &client)?;

        let response: Ticker = client 
            .execute(request) 
            .await? 
            .json() 
            .await 
            .map_err(|e| APIError::RequestFailed(e))?;

        Ok(response)
    }

    pub async fn fetch_filled_orders_pag(&self, product_id: &String) -> Result<Vec<Order>, APIError> {
        let mut url = self.prepare_url(Endpoint::FilledOrders, product_id);
        let client = reqwest::Client::new();
        let mut orders = Vec::<Order>::new();

        loop {
            let request = self.build_coinbase_request(&url, &client)?;
            let response = self.process_request(&client, request).await;
            let mut set_to_break: bool = false;

            match response {
                Ok(response) => {
                    match response.headers().get("CB-AFTER") {
                        Some(val) => {
                            url = self.prepare_url(Endpoint::FilledOrders, product_id)
                                .add_param("after", val.to_str().unwrap());
                            println!("URL: |{}|", url.get());
                        },
                        None => { set_to_break = true; },
                    }
        
                    let data: Vec<Order> = response.json().await.map_err(|e| APIError::RequestFailed(e))?;
                    for order in data {
                        orders.push(order);
                    }
        
                    if set_to_break == true { break; }
                },
                Err(_e) => {
                    println!("Oops");
                    break;
                    //return Err(APIError::BadRequest("Ah"));
                }
            }
        }
        Ok(orders)
    }

    pub async fn fetch_transfers(&self, account_id: &String) -> Result<Vec<Transfer>, APIError> {
        let url = self.prepare_url(Endpoint::Transfers, account_id);
        let client = reqwest::Client::new();
        let request = self.build_coinbase_request(&url, &client)?;

        let response: Vec<Transfer> = self.process_request(&client, request)
            .await
            .unwrap() 
            .json() 
            .await 
            .unwrap();

        Ok(response)
    }

    pub async fn print_response(&self, path: &String) -> Result<String, APIError> {
        const BASE_URL: &str = "https://api.exchange.coinbase.com";
        let url = Url::new(format!("{}{}", BASE_URL, path).as_str());
        let client = reqwest::Client::new();
        let request = self.build_coinbase_request(&url, &client)?;
        let response = client.execute(request).await.map_err(|e| APIError::RequestFailed(e))?;
        Ok(response.text().await?)
    }

    pub async fn fetch_order(&self, order_id: &String) -> Result<Vec<Order>, APIError> {
        let url = self.prepare_url(Endpoint::Order, order_id);
        let client = reqwest::Client::new();
        let request = self.build_coinbase_request(&url, &client)?;

        let response: Vec<Order> = self.process_request(&client, request)
            .await
            .unwrap()
            .json()
            .await 
            .unwrap();

        Ok(response)
    }
}