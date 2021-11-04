use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Team {
    name: String
}

#[derive(Serialize, Deserialize)]
pub struct TeamRel {
    id: String,
    name: String
}
