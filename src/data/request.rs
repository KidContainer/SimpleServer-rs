use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Request {
    pub name: String,
    pub age: i64,
    pub time: i64,
    pub last_time: i64,
    pub extra: Extra,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]

pub struct Extra {
    pub a: String,
    pub b: String,
}
