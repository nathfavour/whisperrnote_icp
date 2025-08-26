// Tags module for whisperrnote_icp backend
use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub struct Tag {
    pub id: Option<String>,
    pub name: Option<String>,
    pub notes: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub usage_count: Option<i64>,
}
