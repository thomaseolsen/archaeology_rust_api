use serde::{Deserialize, Serialize};

/*
 * A struct of the pertinent display fields for Locus
 */
#[derive(Serialize, Deserialize)]
pub struct Locus {
    name: String,
    description: String
}

/*
 * A struct of the relational fields for Locus as it exists in the database
 */
#[derive(Serialize, Deserialize)]
pub struct LocusRel {
    id: String,
    name: String,
    description: String
}
