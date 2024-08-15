use crate::error::Result;

use bson::doc;
use bson::serde_helpers::hex_string_as_object_id;
use futures_util::TryStreamExt;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Account {
    #[serde(rename = "_id", with = "hex_string_as_object_id")]
    id: String,
    username: String,
    password: String,
}

impl Account {
    pub async fn get_all(db: &Database) -> Result<Vec<Account>> {
        // Get the collection of `Account` documents
        let coll: Collection<Account> = db.collection("accounts");
        let cursor = coll.find(doc! {}).await?;
        let accounts: Vec<Account> = cursor.try_collect().await?;

        Ok(accounts)
    }
}
