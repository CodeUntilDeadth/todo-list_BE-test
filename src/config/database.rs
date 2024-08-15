use serde::Deserialize;

serde_with::with_prefix!(pub pdb "db_");

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub name: String,
}
