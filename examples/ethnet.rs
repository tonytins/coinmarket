extern crate coinmarket;

use coinmarket::ethereum::{Web3, Web3Provider};

pub fn main() {
    let address = "0x341A3A994A150962F3e82b195873B736dAEb4bB3";
    let err_msg = "Parsing error";
    let network = Web3::new(Web3Provider::MainNet);
    let balance = network.get_balance(address).expect(err_msg);
    let total_supply = network.get_total_supply().expect(err_msg);
    let last_price = network.get_last_price().expect(err_msg);

    println!("Balance: {:#?}", balance);
    println!("Total supply: {:#?}", total_supply);
    println!("{:#?}", last_price);
}
