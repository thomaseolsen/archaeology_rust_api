use serde::{Deserialize, Serialize};

/*
 * A struct of the pertinent display fields for AscAreaTeam
 */
#[derive(Serialize, Deserialize)]
pub struct AscAreaTeam {
    area_id: String,
    team_id: String
}

/*
 * A struct of the relational fields for AscAreaTeam as it exists in the database
 */
#[derive(Serialize, Deserialize)]
pub struct AscAreaTeamRel {
    id: String,
    area_id: String,
    team_id: String
}
