mod r#impl;

#[derive(Debug, Deserialize)]
pub struct PaymentData {
    pub address: String,
    pub amount: String,
    pub currency: String,
    pub serial_number: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Payment {
    Data(PaymentData),
}
