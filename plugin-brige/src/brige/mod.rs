mod r#impl;

#[derive(Debug, Deserialize)]
pub struct BrigeVout {
    address: String,
    amount: String,
    position: i32,
}

#[derive(Debug, Deserialize)]
pub struct BrigeOutTransaction {
    #[serde(rename = "coinType")]
    pub coin_type: String,
    pub vout: Vec<BrigeVout>,
    pub status: String,
    pub tip: String,
}

#[derive(Debug, Deserialize)]
pub struct BrigeOut {
    #[serde(rename = "orderId")]
    pub tx_id: String,
    #[serde(rename = "txs")]
    pub transactions: Vec<BrigeOutTransaction>,
}

#[derive(Debug, Deserialize)]
pub struct BrigeIn {
    #[serde(rename = "txid")]
    pub tx_id: String,
    #[serde(rename = "coinType")]
    pub coin_type: String,
    pub vout: Vec<BrigeVout>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Brige {
    In(BrigeIn),
    Out(BrigeOut),
}
