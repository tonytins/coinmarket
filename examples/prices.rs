extern crate coinmarket;

use coinmarket::exchange::CoinExchange;

pub fn main() {

    let from_coin = "btc";
    let to_coin = "eth";
    let err_msg = "Parsing error";

    let exchange = CoinExchange::new(from_coin, to_coin);
    let market_info = exchange.get_market_info().expect(err_msg);
    let ex_limit = exchange.get_limit().expect(err_msg);

    println!("the value of {} in {} is {}", from_coin, to_coin, market_info.rate);
    println!("{}'s exchange limit to {} is {}", from_coin, to_coin, ex_limit.limit);

}