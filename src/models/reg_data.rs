use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrganizerData {
    email: String,
    company: String,
    tin: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserData {
    name: String,
    surname: String,
    email: String,
    birthday: String,
    password: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogInData {
    login: String,
    password: String
}