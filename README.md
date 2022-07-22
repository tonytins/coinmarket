# Coin Market

Coin Market was formally used for getting market data from ShapeShift but after changes made in the organization I've since removed those APIs and have deprecated this library. It is still useful for interacting with EVM-compatible networks that use Etherscan. Those remaining APIs have been used in [Web3Scan](https://crates.io/crates/web3scan).

## Installation

```toml
[dependencies]
coinmarket = "0.4"
```

## Examples

### 0.4 and Web3Scan

```env
APIKEY=[key]
```

```rust
// use web3scan::Web3;
use coinmarket::web3::Web3;

pub fn main() {
    let network = Web3::new("api.etherscan.io");
    let balance = network
    .get_balance("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B")
    .expect("Parsing error");

    println!("{}", balance);
}
```

### 0.3 and earlier

```env
ETHSCAN=[key]
```

```rust
// Web3 and Web3Provider are formally Ethereum and EthNetworks, 
// respectively, in versions prior to 0.2.102. I forgot to warn about
// that breaking change but than the pandemic happened. Sorry about that.
use coinmarket::ethereum::{Web3, Web3Provider};

pub fn main() {
    let network = Web3::new(Web3Provider::MainNet);
    let balance = network
    .get_balance("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B")
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
