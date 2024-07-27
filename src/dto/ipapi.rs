use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IPAPI {
    pub ip: String,
    pub country_code: String,
}
