/*
 * This project is licensed under the MIT license.
 * See the LICENSE file in the project root for more information.
 */
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ValidateAddress {
    #[serde(rename(deserialize = "isvalid"))]
    pub is_valid: bool,
}

#[derive(Deserialize, Debug)]
pub struct MarketInfo {
    pub rate: f64,
    #[serde(rename(deserialize = "minerFee"))]
    pub miner_fee: f64,
    pub limit: f64,
    pub minimum: f64,
    #[serde(rename(deserialize = "maxLimit"))]
    pub max_limit: f64
}

#[derive(Deserialize, Debug)]
pub struct ExchangeLimit {
    pub limit: f64,
    pub minimum: f64,
}

#[derive(Debug, Deserialize)]
pub struct Etherscan<T> {
    pub status: String,
    pub message: String,
    pub result: T,
}

#[derive(Debug, Deserialize)]
pub struct EthBalance {
    pub account: String,
    pub balance: String,
}

#[derive(Debug, Deserialize)]
pub struct  EthPrice {
    pub ethbtc: String,
    pub ethbtc_timestamp: String,
    pub ethusd: String,
    pub ethusd_timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct EthTransaction {
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    pub hash: String,
    pub nonce: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(rename = "isError")]
    pub is_error: String,
    pub txreceipt_status: String,
    pub input: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    pub confirmations: String,
}
#[derive(Debug, Deserialize)]
pub struct EthTransactionHash {
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    pub from: String,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub to: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    pub value: String,
    pub v: String,
    pub r: String,
    pub s: String,
}

#[derive(Debug, Deserialize)]
pub struct EthTransactionnReceipt {
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<serde_json::Value>,
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: String,
    pub from: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    #[serde(rename = "Ethlogs")]
    pub ethlogs: Vec<Ethlog>,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,
    pub root: String,
    pub to: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
}

#[derive(Debug, Deserialize)]
pub struct Ethlog {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "logIndex")]
    pub log_index: String,
    pub removed: bool,
}

#[derive(Debug, Deserialize)]
pub struct EthBlockByNumber {
    pub difficulty: String,
    #[serde(rename = "extraData")]
    pub extra_data: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    pub hash: String,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,
    pub miner: String,
    #[serde(rename = "mixHash")]
    pub mix_hash: String,
    pub nonce: String,
    pub number: String,
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: String,
    #[serde(rename = "sha3Uncles")]
    pub sha3_uncles: String,
    pub size: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    pub timestamp: String,
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<serde_json::Value>,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: String,
    pub uncles: Vec<Option<serde_json::Value>>,
}