use serde::{Deserialize, Serialize};

/*
 * A struct of the pertinent display fields for Team
 */
#[derive(Serialize, Deserialize)]
pub struct Team {
    name: String
}

/*
 * A struct of the relational fields for Team as it exists in the database
 */
#[derive(Serialize, Deserialize)]
pub struct TeamRel {
    id: String,
    name: String
}
