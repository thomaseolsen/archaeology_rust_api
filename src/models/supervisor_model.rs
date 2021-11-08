use serde::{Deserialize, Serialize};

/*
 * A struct of the pertinent display fields for Supervisor
 */
#[derive(Serialize, Deserialize)]
pub struct Supervisor {
    first_name: String,
    middle_name: String,
    last_name: String,
    initials: String,
    email: String
}

/*
 * A struct of the relational fields for Supervisor as it exists in the database
 */
#[derive(Serialize, Deserialize)]
pub struct SupervisorRel {
    id: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    initials: String,
    email: String,
    team_id: String
}
