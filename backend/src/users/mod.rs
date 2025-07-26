// User module for whisperrnote_icp backend

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub wallet_address: Option<String>,
    pub created_at: Option<String>, // Use chrono::DateTime in real impl
    pub updated_at: Option<String>, // Use chrono::DateTime in real impl
}
