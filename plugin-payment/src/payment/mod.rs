mod r#impl;

#[derive(Debug, Deserialize)]
pub struct Payment {
    pub address: String,
    pub amount: i64,
    pub currency: String,
    pub serial_number: String,
}
