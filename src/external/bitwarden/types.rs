use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Item {
    Login(Login),
    SecureNote(SecureNote),
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub name: String,
    pub notes: Option<String>,
    pub login: LoginLogin,
}

#[derive(Debug, Deserialize)]
pub struct LoginLogin {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SecureNote {
    pub name: String,
    pub notes: String,
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Login(login) => &login.name,
            Item::SecureNote(secure_note) => &secure_note.name,
        }
    }

    pub fn notes(&self) -> Option<&str> {
        match self {
            Item::Login(login) => login.notes.as_deref(),
            Item::SecureNote(secure_note) => Some(&secure_note.notes),
        }
    }
}
