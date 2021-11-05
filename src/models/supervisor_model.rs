use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Supervisor {
    first_name: String,
    middle_name: String,
    last_name: String,
    initials: String,
    email: String
}

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
