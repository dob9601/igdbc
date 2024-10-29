use serde::{Deserialize, Deserializer};

pub fn deserialize_cover<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct ImageData {
        url: Option<String>,
    }

    let url = <ImageData>::deserialize(deserializer)?.url;

    Ok(url)
}
