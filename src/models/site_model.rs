use serde::{Deserialize, Serialize};

/*
 * A struct of the pertinent display fields for Site
 */
#[derive(Serialize, Deserialize)]
pub struct Site {
    short_name: String,
    full_name: String,
    publication_name: String
}

/*
 * A struct of the relational fields for Site as it exists in the database
 */
#[derive(Serialize, Deserialize)]
pub struct SiteRel {
    id: String,
    short_name: String,
    full_name: String,
    publication_name: String
}
