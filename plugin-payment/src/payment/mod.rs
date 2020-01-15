mod r#impl;

#[derive(Debug, Deserialize)]
pub struct PaymentOut {
    pub serial_number: String,
    pub currency: String,
    pub amount: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct PaymentIn {
    pub serial_number: String,
    pub currency: String,
    pub amount: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Payment {
    In(PaymentIn),
    Out(PaymentOut),
}
