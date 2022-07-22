use crate::models::{ExchangeLimit, MarketInfo, ValidateAddress};
use reqwest::Error;

static API: &str = "https://shapeshift.io/";

#[deprecated(
    since = "0.2.102",
    note = "Due to changes in ShapeShift's organization, this API will no longer available."
)]
pub struct CoinExchange {
    from_coin: String,
    to_coin: String,
}

/// CoinExchange uses ShapeShift's API to gather market information.
/// Some of the documentation provided comes from theirs.
/// https://docs.shapeshift.io
impl CoinExchange {
    pub fn new<S: Into<String>>(from_coin: S, to_coin: S) -> Self
    where
        S: Into<String>,
    {
        CoinExchange {
            from_coin: from_coin.into(),
            to_coin: to_coin.into(),
        }
    }

    pub fn is_valid_address<S: Into<String>>(&self, address: S) -> Result<bool, Error>
    where
        S: Into<String>,
    {
        let request = format!(
            "{}validateAddress/{}/{}",
            API,
            address.into(),
            self.to_coin.to_lowercase()
        );
        let mut response = reqwest::get(&request)?;
        let try_valid: ValidateAddress = response.json()?;

        Ok(try_valid.is_valid)
    }

    pub fn get_limit(&self) -> Result<ExchangeLimit, Error> {
        let request = format!(
            "{}limit/{}_{}",
            API,
            self.from_coin.to_lowercase(),
            self.to_coin.to_lowercase()
        );

        let mut response = reqwest::get(&request)?;
        let limit: ExchangeLimit = response.json()?;

        Ok(limit)
    }

    pub fn get_market_info(&self) -> Result<MarketInfo, Error> {
        let request = format!("{}marketinfo/{}_{}", API, self.from_coin, self.to_coin);
        let mut response = reqwest::get(&request)?;
        let market: MarketInfo = response.json()?;

        Ok(market)
    }
}
