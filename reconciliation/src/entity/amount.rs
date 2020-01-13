use std::str::FromStr;

use num_bigint::{BigInt, ParseBigIntError};
use num_traits::identities::Zero;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Amount {
    inner: BigInt,
}

impl FromStr for Amount {
    type Err = ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigInt::from_str(s).map(|inner| Amount { inner })
    }
}

impl From<i64> for Amount {
    fn from(v: i64) -> Amount {
        Amount {
            inner: BigInt::from(v),
        }
    }
}

impl From<u64> for Amount {
    fn from(v: u64) -> Amount {
        Amount {
            inner: BigInt::from(v),
        }
    }
}

impl Default for Amount {
    fn default() -> Self {
        Amount {
            inner: BigInt::zero(),
        }
    }
}
