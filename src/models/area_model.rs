use serde::{Deserialize, Serialize};

/*
 * A struct of the pertinent display fields for Area
 */
#[derive(Serialize, Deserialize)]
pub struct Area {
    name: String,
    objective: String
}

/*
 * A struct of the relational fields for Area as it exists in the database
 */
#[derive(Serialize, Deserialize)]
pub struct AreaRel {
    id: String,
    site_id: String,
    name: String,
    objective: String
}
