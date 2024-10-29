mod _entities;

mod game;
pub use game::{ActiveModel as GameActive, Column as GameColumn, Entity as Game, IGDBGame};

mod query;
pub use query::{
    ActiveModel as QueryActive, Column as QueryColumn, Entity as Query, Model as QueryModel,
};

mod deserializers;
