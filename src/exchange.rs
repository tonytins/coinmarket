/*
 * This project is licensed under the MIT license.
 * See the LICENSE file in the project root for more information.
 */
use reqwest::Error;
use crate::models::{ExchangeLimit, ValidateAddress, MarketInfo};

static API: &str = "https://shapeshift.io/";

pub struct CoinExchange {
    from_coin: String,
    to_coin: String,
}

/// CoinExchange uses ShapeShift's API to gather market information.
/// Some of the documentation provided comes from theirs.
/// https://docs.shapeshift.io
impl CoinExchange {
    pub fn new<S: Into<String>>(from_coin: S, to_coin: S)
        -> Self where S: Into<String> {
        CoinExchange {
            from_coin: from_coin.into(),
            to_coin: to_coin.into(),
        }
    }

    /// This allows the user to verify that their receiving address is a valid
    /// address according to a given wallet daemon. If isValid returns true, this
    /// address is valid according to the coin daemon indicated by the currency symbol.
     /// ```rust
    /// extern crate coinmarket;
    /// use coinmarket::exchange::CoinExchange;
    ///
    /// pub fn main() {
    ///     let err_msg = "parsing error";
    ///     let exchange = CoinExchange::new("xmr", "eth");
    ///     let valid_address = exchange
    ///            .is_valid_address("0x9De8991C56F622175274fb358f981AF6F903a799")
    ///            .expect(err_msg);
    ///        let invalid_address = exchange
    ///            .is_valid_address("1M8FvUfRG9vtvLA2T63Lo8TPaVMkdFmR7H")
    ///            .expect(err_msg);
    ///
    ///     if valid_address == true {
    ///         println!("valid");
    ///      }
    ///
    ///     if invalid_address == false {
    ///         println!("invalid!");
    ///      }
    /// }
    /// ```
    pub fn is_valid_address<S: Into<String>>(&self, address: S)
        -> Result<bool, Error> where S: Into<String> {
        let request = format!("{}validateAddress/{}/{}", API, address.into(),
                              self.to_coin.to_lowercase());
        let mut response = reqwest::get(&request)?;
        let try_valid: ValidateAddress = response.json()?;

        Ok(try_valid.is_valid)
    }

    /// This gets the current rate offered by ShapeShift. This is an estimate
    /// because the rate can occasionally change rapidly depending on the markets.
    /// The rate is also a usable rate rather than a direct market rate. This means
    /// that multiplying your input coin amount times the rate should give you a
    /// close approximation of what will be sent out. This rate does not include the
    /// transaction (miner) fee taken off of every transaction.
    ///
    /// ## Example
    /// ```rust
    /// extern crate coinmarket;
    /// use coinmarket::exchange::CoinExchange;
    ///
    /// pub fn main() {
    ///     let exchange = CoinExchange::new("btc", "xmr");
    ///     let market_limit = exchange.get_limit().expect("parsing error");
    ///
    ///     println!("{}", market_limit.limit);
    /// }
    /// ```
    pub fn get_limit(&self) -> Result<ExchangeLimit, Error> {
        let request = format!("{}limit/{}_{}", API,
                              self.from_coin.to_lowercase(),
                              self.to_coin.to_lowercase());

        let mut response = reqwest::get(&request)?;
        let limit: ExchangeLimit = response.json()?;

        Ok(limit)
    }

    /// This gets the market info on a specific coin pair.
    ///
    /// ## Example
    /// ```rust
    /// extern crate coinmarket;
    /// use coinmarket::exchange::CoinExchange;
    ///
    /// pub fn main() {
    ///     let exchange = CoinExchange::new("btc", "xmr");
    ///     let market_info = exchange.get_market_info().expect("parsing error");
    ///
    ///     println!("{}", market_info.rate);
    /// }
    /// ```
    pub fn get_market_info(&self) -> Result<MarketInfo, Error> {
        let request = format!("{}marketinfo/{}_{}", API, self.from_coin, self.to_coin);
        let mut response = reqwest::get(&request)?;
        let market: MarketInfo = response.json()?;

        Ok(market)
    }
}

#[cfg(test)]
mod exchange_test {
    use crate::exchange::CoinExchange;

    static FROM_COIN: &str = "btc";
    static TO_COIN: &str = "eth";
    static ERR_MSG: &str = "Parsing error";

    #[test]
    fn valid_address_test() {
        let exchange = CoinExchange::new(FROM_COIN, TO_COIN);
        let valid_address = exchange
            .is_valid_address("0x9De8991C56F622175274fb358f981AF6F903a799")
            .expect(ERR_MSG);
        let invalid_address = exchange
            .is_valid_address("1M8FvUfRG9vtvLA2T63Lo8TPaVMkdFmR7H")
            .expect(ERR_MSG);

        assert!(valid_address);
        assert!(!invalid_address);
    }

    #[test]
    fn market_info_test() {
        let exchange = CoinExchange::new(FROM_COIN, TO_COIN);
        let market_info = exchange.get_market_info()
            .expect(ERR_MSG);

        assert!(!market_info.rate.is_nan());
    }

    #[test]
    fn ex_limit_test()
    {
        let exchange = CoinExchange::new(FROM_COIN, TO_COIN);
        let ex_limit = exchange.get_limit()
            .expect(ERR_MSG);

        assert!(!ex_limit.limit.is_nan());
    }
}

