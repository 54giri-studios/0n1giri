use diesel::prelude::*;
use std::borrow::Cow;
use chrono::{DateTime, Utc};

use crate::{Guild, Role};

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewChannel<'a> {
    guild_id: i32,
    name: Cow<'a, str>,
    kind: Cow<'a, str>
}

/// Represents a generic channel.
/// Mirrors [crate::schema::channels] in the database
#[derive(Debug, Serialize, Insertable, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel {
    /// It's globally unique id
    pub id: i32,
    /// The guild's id that it belongs to
    /// Must refer to an actual [crate::Guild]
    pub guild_id: i32,
    /// It's display name
    pub name: String,
    /// The kind of the channel
    /// Must refer to an actual [crate::ChannelKind]
    pub kind: String
}

/// An enum like defining a [Channel]'s kind.
/// Mirrors [crate::schema::channel_kinds]
#[derive(Debug, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::channel_kinds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelKind<'a> {
    /// The kind of the channel
    /// Must be constructed through it's methods
    kind: Cow<'a, str>
}

impl<'a> ChannelKind<'a> {
    fn new(kind: &'a str) -> Self {
        Self { kind: kind.into() }
    }

    /// A text channel: Users can write into it
    pub fn text() -> Self {
        Self::new("text")
    }

    /// A category channel: Used to group channels together
    pub fn category() -> Self {
        Self::new("category")
    }

    /// A voice channel: Can be connected to
    pub fn voice() -> Self {
        Self::new("voice")
    }

    /// A system channel: used by the server to send data
    pub fn system() -> Self {
        Self::new("system")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryConfig {
    pub limit: Option<i32>,
    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::channel_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelPermissions {
    role_id: i32,
    guild_id: i32,
    channel_id: i32,
    can_read: bool,
    can_write: bool,
}

#[derive(Debug, Serialize)]
pub struct PopulatedChannelPermissions {
    #[serde(flatten)]
    permissions: ChannelPermissions,
    role: Role,
    guild: Guild,
    channel: Channel,
}

impl PopulatedChannelPermissions {
    pub fn new(
        permissions: ChannelPermissions,
        role: Role,
        guild: Guild,
        channel: Channel
    ) -> Self {
        Self { permissions, role, guild, channel }
    }
}