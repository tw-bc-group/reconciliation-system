mod r#impl;

#[derive(Debug, Deserialize)]
pub struct BridgeVout {
    address: String,
    amount: String,
    position: i32,
}

#[derive(Debug, Deserialize)]
pub struct BridgeOutTransaction {
    #[serde(rename = "coinType")]
    pub coin_type: String,
    pub vout: Vec<BridgeVout>,
    pub status: String,
    pub tip: String,
    #[serde(rename = "blockTime")]
    pub block_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct BridgeOut {
    #[serde(rename = "orderId")]
    pub tx_id: String,
    #[serde(rename = "txs")]
    pub transactions: Vec<BridgeOutTransaction>,
}

#[derive(Debug, Deserialize)]
pub struct BridgeIn {
    #[serde(rename = "txid")]
    pub tx_id: String,
    #[serde(rename = "coinType")]
    pub coin_type: String,
    pub vout: Vec<BridgeVout>,
    #[serde(rename = "blockTime")]
    pub block_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Bridge {
    In(BridgeIn),
    Out(BridgeOut),
}
