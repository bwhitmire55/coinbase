use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub ending_before: Option<String>,
    pub starting_after: Option<String>,
    pub limit: i32,
    pub order: String,
    pub previous_uri: Option<String>,
    pub next_uri: Option<String>,
}