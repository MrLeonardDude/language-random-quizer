use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub nome: String,
    pub login: String,
    pub psswd: String,
    pub date_inserted: Option<NaiveDateTime>,
    pub date_updated: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub nome: String,
    pub login: String,
    pub psswd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub login: String,
    pub psswd: String,
}

