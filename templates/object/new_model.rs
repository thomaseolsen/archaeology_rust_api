use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewObject {
    name: String,
    description: String
}

#[derive(Serialize, Deserialize)]
pub struct NewObjectRel {
    id: String,
    name: String,
    description: String
}
