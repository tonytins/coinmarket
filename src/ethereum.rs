use crate::models::*;
use reqwest::Error;
use std::env;
use std::path::Path;

fn get_api_key() -> String {
    if Path::new(".env").exists() {
        dotenv::dotenv().expect("Error parsing .env file!");
    }

    let api_key = env::var("ETHSCAN").expect("Failed to locate API key!");

    return format!("&apikey={}", api_key);
}

#[derive(Copy, Clone, Debug)]
pub enum Web3Provider {
    MainNet,
    Ropsten,
    Rinkeby,
    Kovan,
}

#[deprecated(
    since = "0.3.100",
    note = "Please use the Web3 module. This will removed in 0.4.100."
)]
pub struct Web3 {
    provider: Web3Provider,
}

impl Web3 {
    pub fn new(provider: Web3Provider) -> Self {
        Web3 { provider }
    }

    /// Information is provided by Etherscan.io APIs
    fn get_network(&self, network: Web3Provider) -> String {
        let api = "etherscan.io/api?module=";
        let mainnet = format!("https://api.{}", api);
        let rosten = format!("https://api-ropsten.{}", api);
        let kovan = format!("https://api-kovan.{}", api);
        let rinkeby = format!("https://api-rinkeby.{}", api);

        match network {
            Web3Provider::MainNet => mainnet,
            Web3Provider::Ropsten => rosten,
            Web3Provider::Kovan => kovan,
            Web3Provider::Rinkeby => rinkeby,
        }
    }

    /// Retrieves the client's Ethereum balance as Wei.
    ///
    /// ## Example
    /// ```rust
    /// extern crate coinmarket;
    /// use coinmarket::ethereum::{Web3, Web3Provider};
    ///
    /// fn main() {
    ///
    /// let network = Web3::new(Web3Provider::MainNet);
    /// let balance = network.get_balance("0x9De8991C56F622175274fb358f981AF6F903a799")
    ///                 .expect("parser error");
    ///
    /// println!("{}", balance);
    ///
    /// }
    /// ```
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

    /// Retrieves the client's Ethereum balance as Wei.
    ///
    /// ## Example
    /// ```rust
    /// extern crate coinmarket;
    /// use coinmarket::ethereum::{Web3, Web3Provider};
    ///
    /// fn main() {
    ///
    /// let network = Web3::new(Web3Provider::MainNet);
    /// let total_supply = network.get_total_supply().expect("parser error");
    ///
    /// println!("{}", total_supply);
    ///
    /// }
    /// ```
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

    /// Retrieves the client's Ethereum balance as Wei.
    ///
    /// ## Example
    /// ```rust
    /// extern crate coinmarket;
    /// use coinmarket::ethereum::{Web3, Web3Provider};
    ///
    /// fn main() {
    ///
    /// let network = Web3::new(Web3Provider::MainNet);
    /// let last_price = network.get_last_price().expect("parser error");
    ///
    /// println!("{}", last_price);
    ///
    /// }
    /// ```
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
