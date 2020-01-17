use std::{
    fmt::{self, Display},
    str::FromStr,
};

use num_bigint::{BigInt, ParseBigIntError};
use num_traits::identities::Zero;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Amount {
    inner: BigInt,
}

impl Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
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
