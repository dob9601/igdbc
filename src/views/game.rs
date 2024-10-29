use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A game that has been pulled from [IGDB](https://www.igdb.com/) and restructured to better suit
/// the needs of OmniLFG
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, PartialEq)]
pub struct GameJson {
    /// The ID of this game, as per IGDB
    pub id: u32,

    pub name: String,

    /// A brief summary describing what this game is about
    pub summary: Option<String>,

    /// A value from 0 to 1 representing the average rating this game has
    pub aggregated_rating: Option<f32>,

    /// A list of themes that this game contains
    pub themes: Option<Vec<String>>,

    /// A link to this game's [IGDB](https://www.igdb.com/) page
    pub url: String,

    pub artwork_url: Option<String>,

    pub cover_art_url: Option<String>,

    /// The date at which this game was first released
    pub first_release_date: Option<NaiveDateTime>,

    /// The name of the franchise that this game belongs to
    pub franchise: Option<String>,

    /// A list of genres that describe this game
    pub genres: Option<Vec<String>>,

    /// A list of game modes that this game supports
    pub game_modes: Option<Vec<String>>,

    /// Whether or not this game supports online multiplayer. This, unfortunately, is not 100%
    /// accurate - if it was, we would use this to filter down games further
    pub supports_online_multiplayer: Option<bool>,

    /// A list of platforms that this game is available on
    pub platforms: Option<Vec<String>>,
}
