use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Site {
    short_name: String,
    full_name: String,
    publication_name: String
}

#[derive(Serialize, Deserialize)]
pub struct SiteRel {
    id: String,
    short_name: String,
    full_name: String,
    publication_name: String
}
