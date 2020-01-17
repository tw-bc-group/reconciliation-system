use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    In,
    Out,
    Unknown,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Unknown
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let s = match *self {
            Direction::In => "入金",
            Direction::Out => "出金",
            Direction::Unknown => "未知",
        };
        write!(f, "{}", s)
    }
}
