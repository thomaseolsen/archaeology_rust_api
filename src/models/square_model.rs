use serde::{Deserialize, Serialize};

/*
 * A struct of the pertinent display fields for Square
 */
#[derive(Serialize, Deserialize)]
pub struct Square {
    name: String,
    description: String
}

/*
 * A struct of the relational fields for Square as it exists in the database
 */
#[derive(Serialize, Deserialize)]
pub struct SquareRel {
    id: String,
    name: String,
    description: String
}
