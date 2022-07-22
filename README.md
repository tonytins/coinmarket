# Coin Market

Coin Market is a simple library designed to make it easy get market or account information for cryptocurrencies by interacting with ShapeShift and Etherscan APIs.

## DEPRECATION NOTICE

- The Ethereum module has been deprecated and will replaced by a generic Web3 API to accommodate newer EVM-based networks.
- Version 0.4 will be the **final** release.

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
    let network = Web3::new("api.etherscan.io");
    let balance = network
    .ether_balance("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B")
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
