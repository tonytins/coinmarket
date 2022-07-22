use crate::models::*;
use reqwest::Error;
use std::env;
use std::path::Path;

fn get_api_key() -> String {
    if Path::new(".env").exists() {
        dotenv::dotenv().expect("Error parsing .env file!");
    }

    let api_key = env::var("APIKEY").expect("Failed to locate API key!");

    return format!("&apikey={}", api_key);
}

pub struct Web3 {
    provider: String,
}

impl Web3 {
    pub fn new(provider: String) -> Self {
        Web3 { provider }
    }

    pub fn get_balance<S: Into<String>>(&self, address: S) -> Result<String, Error>
    where
        S: Into<String>,
    {
        let request = format!(
            "{}account&action=balance&address={}&tag=latest{}",
            self.get_network(self.provider),
            address.into(),
            get_api_key().as_str()
        );
        let mut response = reqwest::get(&request)?;
        let balance: Etherscan<String> = response.json()?;

        Ok(balance.result)
    }

    pub fn get_total_supply(&self) -> Result<String, Error> {
        let request = format!(
            "{}stats&action=ethsupply{}",
            self.get_network(self.provider),
            get_api_key().as_str()
        );
        let mut response = reqwest::get(&request)?;
        let taken_supply: Etherscan<String> = response.json()?;

        Ok(taken_supply.result)
    }

    pub fn get_last_price(&self) -> Result<EthPrice, Error> {
        let request = format!(
            "{}stats&action=ethprice{}",
            self.get_network(self.provider),
            get_api_key().as_str()
        );
        let mut response = reqwest::get(&request)?;
        let last_price: Etherscan<EthPrice> = response.json()?;
        Ok(last_price.result)
    }

    pub fn get_transactions<S: Into<String>>(
        &self,
        address: S,
        start_block: i64,
        end_block: i64,
    ) -> Result<Vec<EthTransaction>, Error>
    where
        S: Into<String>,
    {
        let request = format!(
            "{}account&action=txlist&address={}&startblock={}&endblock={}&sort=asc{}",
            self.get_network(self.provider),
            address.into(),
            start_block,
            end_block,
            get_api_key().as_str()
        );

        let mut response = reqwest::get(&request)?;
        let balance: Etherscan<Vec<EthTransaction>> = response.json()?;

        Ok(balance.result)
    }
}
