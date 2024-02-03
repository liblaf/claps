use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Item {
    Login(ItemLogin),
    SecureNote(ItemSecureNote),
}

#[derive(Deserialize)]
pub struct ItemLogin {
    pub name: String,
    pub notes: Option<String>,
    pub login: LoginLogin,
}

#[derive(Deserialize)]
pub struct LoginLogin {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSecureNote {
    pub name: String,
    pub notes: Option<String>,
    pub secure_note: Value,
}
