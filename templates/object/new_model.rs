use serde::{Deserialize, Serialize};

/*
 * A struct of the pertinent display fields for newobject
 */
#[derive(Serialize, Deserialize)]
pub struct NewObject {
    name: String,
    description: String
}

/*
 * A struct of the relational fields for newobject as it exists in the database
 */
#[derive(Serialize, Deserialize)]
pub struct NewObjectRel {
    id: String,
    name: String,
    description: String
}
