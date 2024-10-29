use serde::Deserialize;

use super::deserializers::deserialize_cover;

#[derive(Deserialize, Debug)]
pub struct IgdbGame {
    id: i32,
    name: String,
    url: String,
    summary: Option<String>,

    #[serde(deserialize_with = "deserialize_cover", default)]
    cover_url: Option<String>,
}
