use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct FilterOptionsModel {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}