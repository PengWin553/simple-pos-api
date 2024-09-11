use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
}