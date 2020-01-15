use crate::entity::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct StatementOneResult {
    name: String,
    data: FlushData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementResult {
    OneSide(StatementOneResult),
    //bridge and payment
    DataMismatch(Vec<StatementOneResult>, Vec<FlushDataMismatch>),
}

impl<'a> From<&'a FlushData> for StatementOneResult {
    fn from(data: &'a FlushData) -> StatementOneResult {
        StatementOneResult {
            name: data.belongs.clone(),
            data: data.to_owned(),
        }
    }
}
