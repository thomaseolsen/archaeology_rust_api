use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Site {
    name: String,
    objective: String
}

#[derive(Serialize, Deserialize)]
pub struct SiteRel {
    id: String,
    site_id: String,
    name: String,
    objective: String
}
