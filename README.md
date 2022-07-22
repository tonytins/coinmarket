# Coin Market

Coin Market is a simple library designed to make it easy get market or account information for cryptocurrencies by interacting with ShapeShift and Etherscan APIs.

## DEPRECATION NOTICE

- Because of changes made in ShapeShift's organization, I've deprecated the CoinExchange module and will no longer available after 0.3.
- The Ethereum module will be deprecated in 0.4 and replaced by a generic Web3 API to accommodate newer EVM-based networks.
- Version 0.5 will be the **final** release.

## Installation

```toml
[dependencies]
coinmarket = "0.3"
```

## Examples

### EVM account balance

```rust
use coinmarket::web3::{Ethereum, EthNetworks};

pub fn main() {
    let network = Web3::new(Web3Provider::MainNet);
    let balance = network.ether_balance("0x341A3A994A150962F3e82b195873B736dAEb4bB3")
        .expect("Parsing error");

    println!("{}", balance);
}
```

## Requirements

- Rust 2021 Edition or later
- IDEs or Editors
  - [Visual Studio Code](https://code.visualstudio.com/)
  - [InteliJ IDEA](https://www.jetbrains.com/idea/) or [CLion](https://www.jetbrains.com/clion/)

## License

This project is dual-licensed under the [BSD-3-Clause](COPYING) or the [UNLICENSE](UNLICENSE).
