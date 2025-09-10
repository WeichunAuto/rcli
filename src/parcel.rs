use serde::{Deserialize, Serialize};

/**
 * 包裹订单的数据结构
 */
#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct ParcelOrder {
    #[serde(skip_deserializing)] // 读 csv 时不会填充映射，写时不跳过
    pub index: usize,
    pub order_number: String,
    create_date: String,
    status: Status,
    pub dispatch_site: DispatchSite,
    #[serde(skip)] // 序列化和反序列化时 都skip
    pub site_code: String,
    recipient_name: String,
    value: f32,
}

/// 派件网点枚举
#[derive(Debug, Serialize, Deserialize)]
pub enum DispatchSite {
    AucklandDS,
    Tauranga,
    Hamilton,
    Napier,
    Hastings,
    Rotorua,
    Unknown(String),
}

impl DispatchSite {
    // 返回网点简码
    pub fn site_code(&self) -> String {
        match self {
            DispatchSite::AucklandDS => "AKL".to_string(),
            DispatchSite::Hamilton => "HMT".to_string(),
            DispatchSite::Hastings => "HST".to_string(),
            DispatchSite::Napier => "NPL".to_string(),
            DispatchSite::Rotorua => "RTR".to_string(),
            DispatchSite::Tauranga => "TRG".to_string(),
            DispatchSite::Unknown(_) => "Unknow".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    #[serde(rename = "是")]
    Yes,
    #[serde(rename = "否")]
    No,
}
