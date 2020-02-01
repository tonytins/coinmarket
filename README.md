# Coin Market

![Rust](https://github.com/tonytins/coinmarket/workflows/Rust/badge.svg) [![Build Status](https://travis-ci.org/tonytins/coinmarket.svg?branch=master)](https://travis-ci.org/tonytins/coinmarket) [![Build status](https://ci.appveyor.com/api/projects/status/ffru6ik26j2b87ko?svg=true)](https://ci.appveyor.com/project/tonytins/coinmarket) [![codecov](https://codecov.io/gh/tonytins/coinmarket/branch/master/graph/badge.svg)](https://codecov.io/gh/tonytins/coinmarket)

Coin Market is a simple library designed to make it easy get market or account information for cryptocurrencies by interacting with ShapeShift and Etherscan.io.

## Installation

```toml
[dependencies]
coinmarket = "0.2"
```

### Development

```toml
[dependencies]
coinmarket = { git = "https://github.com/tonytins/coinmarket" }
```

## Example
### Exchange information
```rust
 extern crate coinmarket;
 use coinmarket::exchange::CoinExchange;

 pub fn main() {
      // Tell the exchange we want to pair against Bitcoin and Monero
     let exchange = CoinExchange::new("btc", "xmr");
     // Get the market info of Bitcoin in Monero
     let market_info = exchange.get_market_info().expect("parsing error");

     // Print the market rate
     println!("{}", market_info.rate);
}
``` 
### Ethereum account balance
```rust
extern crate coinmarket;
use coinmarket::ethereum::{Ethereum, EthNetworks};

pub fn main() {
    let network = Web3::new(Web3Provider::MainNet);
    let balance = network.ether_balance("0x341A3A994A150962F3e82b195873B736dAEb4bB3")
        .expect("Parsing error");

    println!("{}", balance);
}
```

## Requirements

### Prerequisites

- Rust 1.41+

## Authors

- **Anthony Foxclaw** - _Initial work_ - [tonytins](https://github.com/tonytins)

See also the list of [contributors](https://github.com/tonytins/isow/contributors) who participated in this project.

## License

This project is licensed under the MIT license - see the [LICENSE](LICENSE) file for details.
