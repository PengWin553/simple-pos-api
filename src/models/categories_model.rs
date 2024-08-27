use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryModel {
    pub category_id: Option<String>,
    pub category_name: Option<String>,
}